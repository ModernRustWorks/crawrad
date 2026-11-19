//! Stdio MCP bridge for CrawRAD IDE.
//!
//! Spawns as a lightweight CLI proxy between an external AI agent speaking MCP
//! over stdio and the live in-process IDE MCP server via local IPC (Named Pipe / Unix socket).
//!
//! `--groups design,debug` (or `CRAWRAD_MCP_GROUPS`) narrows what the agent
//! sees to those toolsets — one IDE server, several small connections.
#![forbid(unsafe_code)]

use std::io::{BufRead, Write};

fn main() {
    let mut instance_id = "default".to_string();
    let mut groups_spec = std::env::var("CRAWRAD_MCP_GROUPS").unwrap_or_default();

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if (args[i] == "--instance" || args[i] == "-i") && i + 1 < args.len() {
            instance_id = args[i + 1].clone();
            i += 2;
        } else if args[i].starts_with("--instance=") {
            instance_id = args[i]["--instance=".len()..].to_string();
            i += 1;
        } else if args[i] == "--groups" && i + 1 < args.len() {
            groups_spec = args[i + 1].clone();
            i += 2;
        } else if let Some(spec) = args[i].strip_prefix("--groups=") {
            groups_spec = spec.to_string();
            i += 1;
        } else {
            i += 1;
        }
    }

    if instance_id == "default"
        && let Ok(env_inst) = std::env::var("CRAWRAD_INSTANCE")
        && !env_inst.is_empty()
    {
        instance_id = env_inst;
    }

    let mut toolset = match crawrad_mcp::Toolset::parse(&groups_spec) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("crawrad-mcp-bridge: {e}");
            std::process::exit(2);
        }
    };

    // Connect to the running IDE instance
    let ipc_stream = match crawrad_os::ipc::IpcStream::connect(&instance_id) {
        Ok(s) => s,
        Err(_) => {
            // IDE is offline: process stdin and answer requests with error, dropping notifications
            let stdin = std::io::stdin();
            let mut reader = stdin.lock();
            let stdout = std::io::stdout();
            let mut writer = stdout.lock();
            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line) {
                if n == 0 {
                    break;
                }
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    respond_offline_if_request(&mut writer, trimmed, &instance_id);
                }
                line.clear();
            }
            return;
        }
    };

    // Connected: single-threaded synchronous bridge to avoid Windows named-pipe deadlocks.
    // Writes request to pipe and, if the JSON-RPC message contains an "id", synchronously
    // waits for and outputs the single-line response from the server.
    let mut pipe_reader = std::io::BufReader::new(ipc_stream);
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    let mut line = String::new();

    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        // Answer exactly when the IDE answers, or every answer after this
        // line goes to the wrong request (see `crawrad_mcp::route`).
        let (has_id, request) = match crawrad_mcp::route::route(trimmed) {
            crawrad_mcp::route::Route::Forward {
                expects_answer,
                request,
            } => (expects_answer, Some(request)),
            crawrad_mcp::route::Route::AnswerHere(answer) => {
                let _ = writeln!(writer, "{answer}");
                let _ = writer.flush();
                line.clear();
                continue;
            }
        };
        if let Some(refusal) = request.as_ref().and_then(|r| toolset.refuse_call(r)) {
            let _ = writeln!(writer, "{refusal}");
            let _ = writer.flush();
            line.clear();
            continue;
        }
        let is_list = request
            .as_ref()
            .and_then(|r| r.get("method"))
            .and_then(serde_json::Value::as_str)
            == Some("tools/list");

        if pipe_reader.get_mut().write_all(line.as_bytes()).is_err()
            || pipe_reader.get_mut().flush().is_err()
        {
            break;
        }

        if has_id {
            let mut resp_line = String::new();
            if pipe_reader.read_line(&mut resp_line).is_err() || resp_line.is_empty() {
                break;
            }
            if is_list
                && let Ok(mut resp) = serde_json::from_str::<serde_json::Value>(resp_line.trim())
            {
                toolset.filter_list_response(&mut resp);
                resp_line = format!("{resp}\n");
            }
            let _ = writer.write_all(resp_line.as_bytes());
            let _ = writer.flush();
        }

        line.clear();
    }

    let _ = writer.flush();
}

fn respond_offline_if_request(writer: &mut impl Write, raw_request: &str, instance_id: &str) {
    let parsed: serde_json::Value = match serde_json::from_str(raw_request) {
        Ok(v) => v,
        Err(_) => return,
    };

    // JSON-RPC 2.0: Only respond if "id" is present! Notifications MUST NOT be responded to.
    let Some(id_val) = parsed.get("id").cloned() else {
        return;
    };

    let err_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id_val,
        "error": {
            "code": -32000,
            "message": format!("CrawRAD IDE is not running or MCP server is unavailable for instance '{instance_id}'")
        }
    });

    if let Ok(msg) = serde_json::to_string(&err_resp) {
        let _ = writeln!(writer, "{msg}");
        let _ = writer.flush();
    }
}
