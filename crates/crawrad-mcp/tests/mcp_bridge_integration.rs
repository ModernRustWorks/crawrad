//! Integration tests for the stdio MCP bridge binary (`crawrad-mcp-bridge`).

use std::io::{BufRead, Read, Write};
use std::process::{Command, Stdio};

use serde_json::Value;

#[test]
fn test_bridge_stdout_first_byte_is_bracket_and_no_bom() {
    let bin = env!("CARGO_BIN_EXE_crawrad-mcp-bridge");
    let instance = format!("test-offline-{}", std::process::id());
    let mut child = Command::new(bin)
        .arg(format!("--instance={instance}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn crawrad-mcp-bridge");

    {
        let mut stdin = child.stdin.take().expect("child stdin");
        stdin
            .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\"}\n")
            .expect("write initialize");
        stdin.flush().expect("flush stdin");
    }

    let mut stdout = child.stdout.take().expect("child stdout");
    let mut first_bytes = [0u8; 1];
    stdout
        .read_exact(&mut first_bytes)
        .expect("read first byte");

    // First byte on stdout MUST be '{' (no BOM, no greeting banner)
    assert_eq!(
        first_bytes[0], b'{',
        "First byte on stdout MUST be '{{', without BOM or greeting text"
    );

    let mut rest = Vec::new();
    stdout.read_to_end(&mut rest).expect("read rest");
    let full = [first_bytes.to_vec(), rest].concat();
    let text = String::from_utf8_lossy(&full);

    // Verify response is a valid JSON-RPC error since IDE is offline
    let val: Value = serde_json::from_str(text.trim()).expect("valid json response");
    assert_eq!(val["jsonrpc"], "2.0");
    assert_eq!(val["id"], 1);
    assert_eq!(val["error"]["code"], -32000);

    let _ = child.kill();
    let _ = child.wait();
}

#[test]
fn test_bridge_notification_produces_no_stdout_frame() {
    let bin = env!("CARGO_BIN_EXE_crawrad-mcp-bridge");
    let instance = format!("test-notif-{}", std::process::id());
    let mut child = Command::new(bin)
        .arg(format!("--instance={instance}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn crawrad-mcp-bridge");

    {
        let mut stdin = child.stdin.take().expect("child stdin");
        stdin
            .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
            .expect("write notification");
        stdin.flush().expect("flush stdin");
    }

    let mut stdout = child.stdout.take().expect("child stdout");
    let mut bytes = Vec::new();
    stdout.read_to_end(&mut bytes).expect("read stdout");
    assert!(
        bytes.is_empty(),
        "Bridge MUST produce zero response frames for notifications"
    );

    let _ = child.kill();
    let _ = child.wait();
}

#[test]
fn test_bridge_online_e2e_communication_no_deadlock() {
    let bin = env!("CARGO_BIN_EXE_crawrad-mcp-bridge");
    let instance = format!("test-e2e-{}", std::process::id());

    // Start a server on instance pipe simulating in-IDE MCP server
    let listener = crawrad_os::ipc::IpcListener::bind(&instance).expect("bind ipc listener");

    let server_handle = std::thread::spawn(move || {
        if let Ok(mut stream) = listener.accept() {
            use std::io::{BufRead, BufReader};
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            while let Ok(n) = reader.read_line(&mut line) {
                if n == 0 {
                    break;
                }
                let trimmed = line.trim();
                if let Ok(val) = serde_json::from_str::<Value>(trimmed)
                    && let Some(id) = val.get("id")
                {
                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": { "status": "ok", "tools": [] }
                    });
                    let _ = stream.write_all(resp.to_string().as_bytes());
                    let _ = stream.write_all(b"\n");
                    let _ = stream.flush();
                }
                line.clear();
            }
        }
    });

    let mut child = Command::new(bin)
        .arg(format!("--instance={instance}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn crawrad-mcp-bridge");

    let mut child_stdin = child.stdin.take().expect("stdin");
    let mut child_stdout = std::io::BufReader::new(child.stdout.take().expect("stdout"));

    // Send initialize request
    child_stdin
        .write_all(b"{\"jsonrpc\":\"2.0\",\"id\":100,\"method\":\"initialize\"}\n")
        .expect("write request");
    child_stdin.flush().expect("flush stdin");

    // Read response line (must arrive promptly, not hang for 20s)
    let mut resp_line = String::new();
    child_stdout
        .read_line(&mut resp_line)
        .expect("read response");
    assert!(!resp_line.is_empty(), "Response must not be empty");

    let val: Value = serde_json::from_str(resp_line.trim()).expect("valid json response");
    assert_eq!(val["id"], 100);
    assert_eq!(val["result"]["status"], "ok");

    // Send notification (must produce zero responses)
    child_stdin
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}\n")
        .expect("write notif");
    child_stdin.flush().expect("flush notif");

    // Close stdin to stop bridge cleanly
    drop(child_stdin);
    let _ = child.wait();
    let _ = server_handle.join();
}

/// A fake IDE that answers by the real IDE's rule (`mcp_host`): a line it
/// cannot read as a request gets `-32700`, a request gets its own id back, a
/// notification gets nothing.
fn fake_ide(instance: &str) -> std::thread::JoinHandle<()> {
    let listener = crawrad_os::ipc::IpcListener::bind(instance).expect("bind ipc listener");
    std::thread::spawn(move || {
        use std::io::{BufRead, BufReader};
        let Ok(mut stream) = listener.accept() else {
            return;
        };
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut line = String::new();
        while reader.read_line(&mut line).unwrap_or(0) > 0 {
            let answer = match serde_json::from_str::<crawrad_mcp::protocol::JsonRpcRequest>(
                line.trim(),
            ) {
                Err(_) => Some(
                    serde_json::json!({"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"from the IDE"}}),
                ),
                Ok(req) => req.id.map(
                    |id| serde_json::json!({"jsonrpc":"2.0","id":id,"result":{"answered":id}}),
                ),
            };
            if let Some(a) = answer {
                let _ = stream.write_all(format!("{a}\n").as_bytes());
                let _ = stream.flush();
            }
            line.clear();
        }
    })
}

/// **After a bad line, every answer still goes to its own request.** Before,
/// the IDE's `-32700` for the bad line stayed in the pipe and the next
/// request got it as its answer — and the one after got the previous one's.
/// A `"id": null` line made the bridge wait for an answer that never comes.
#[test]
fn a_bad_line_does_not_shift_the_answers() {
    let bin = env!("CARGO_BIN_EXE_crawrad-mcp-bridge");
    let instance = format!("test-shift-{}", std::process::id());
    let server = fake_ide(&instance);

    let mut child = Command::new(bin)
        .arg(format!("--instance={instance}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn crawrad-mcp-bridge");
    let mut stdin = child.stdin.take().expect("stdin");
    let stdout = child.stdout.take().expect("stdout");

    // Lines out of the bridge, read on a thread so a hang is a failure, not a
    // stuck test.
    let (tx, rx) = std::sync::mpsc::channel::<Value>();
    std::thread::spawn(move || {
        use std::io::BufRead;
        for line in std::io::BufReader::new(stdout)
            .lines()
            .map_while(Result::ok)
        {
            if let Ok(v) = serde_json::from_str(&line) {
                let _ = tx.send(v);
            }
        }
    });
    let next = || rx.recv_timeout(std::time::Duration::from_secs(5));

    let send = |stdin: &mut std::process::ChildStdin, line: &str| {
        stdin.write_all(format!("{line}\n").as_bytes()).unwrap();
        stdin.flush().unwrap();
    };
    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"path":"C:\d"}}"#,
    );
    let first = next().expect("an answer to the bad line");
    assert_eq!(first["error"]["code"], -32700);

    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":null,"method":"tools/list"}"#,
    );
    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":7,"method":"tools/list"}"#,
    );
    let second = next().expect("the answer to request 7");
    assert_eq!(second["id"], 7, "{second}");
    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":8,"method":"tools/list"}"#,
    );
    let third = next().expect("the answer to request 8");
    assert_eq!(third["id"], 8, "{third}");

    drop(stdin);
    let _ = child.wait();
    let _ = server.join();
}
