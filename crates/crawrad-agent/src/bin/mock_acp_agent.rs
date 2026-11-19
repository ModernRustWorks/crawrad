//! Mock ACP Agent subprocess for testing the Agent Client Protocol.
#![forbid(unsafe_code)]

use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_REQ_ID: AtomicU64 = AtomicU64::new(1000);

fn main() {
    // `--linger`: outlive the end of stdin, as `node codex-acp` does, so a test
    // can tell whether its parent's death really ends it (process tree)
    let linger = std::env::args().any(|a| a == "--linger");
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut lines = BufReader::new(stdin.lock()).lines();

    let mut recorded_mcp_servers = Vec::new();
    let mut recorded_session_ids = Vec::new();
    let mut current_model = "mock-fast".to_string();
    let mut current_effort = "medium".to_string();

    let make_config_options = |model: &str, effort: &str| -> Vec<serde_json::Value> {
        vec![
            serde_json::json!({
                "id": "model",
                "name": "Model",
                "category": "model",
                "type": "select",
                "currentValue": model,
                "options": [
                    { "value": "mock-fast", "name": "Mock Fast" },
                    { "value": "mock-smart", "name": "Mock Smart" }
                ]
            }),
            serde_json::json!({
                "id": "effort",
                "name": "Reasoning Effort",
                "category": "thought_level",
                "type": "select",
                "currentValue": effort,
                "options": [
                    { "value": "low", "name": "Low" },
                    { "value": "medium", "name": "Medium" },
                    { "value": "high", "name": "High" }
                ]
            }),
        ]
    };

    while let Some(line) = lines.next() {
        let Ok(l) = line else { break };
        let trimmed = l.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) else {
            continue;
        };

        // If this is a response to an agent request (e.g. fs/read_text_file)
        if val.get("method").is_none() && val.get("id").is_some() {
            continue;
        }

        let method = val.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let id = val.get("id").and_then(|i| i.as_u64());

        match method {
            "initialize" => {
                let params = val.get("params");
                let client_caps = params.and_then(|p| p.get("clientCapabilities"));
                assert!(
                    client_caps.is_some(),
                    "clientCapabilities is required in initialize handshake"
                );

                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": 1,
                        "agentInfo": {
                            "name": "MockAcpAgent",
                            "version": "1.0.0"
                        },
                        "agentCapabilities": {
                            "loadSession": true,
                            "promptCapabilities": {
                                "image": true
                            }
                        }
                    }
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
            }
            "session/new" => {
                if let Some(params) = val.get("params") {
                    let cwd = params.get("cwd").and_then(|c| c.as_str()).unwrap_or("");
                    assert!(!cwd.is_empty(), "cwd is required in session/new");
                    let cwd_path = std::path::Path::new(cwd);
                    assert!(
                        cwd_path.is_absolute(),
                        "cwd must be absolute in session/new"
                    );

                    if let Some(servers) = params.get("mcpServers").and_then(|s| s.as_array()) {
                        for s in servers {
                            assert!(s.get("env").is_some(), "env is required in McpServerStdio");
                            let cmd = s.get("command").and_then(|c| c.as_str()).unwrap_or("");
                            assert!(!cmd.is_empty(), "command is required in McpServerStdio");
                            let cmd_path = std::path::Path::new(cmd);
                            assert!(
                                cmd_path.is_absolute(),
                                "command must be absolute in McpServerStdio"
                            );
                        }
                        recorded_mcp_servers = servers.clone();
                    }
                }
                // the process id in it, as a real agent's sessions are its own: a
                // new conversation (a new process) must not look like the old one
                let sid = format!(
                    "mock-session-{}-{}",
                    std::process::id(),
                    NEXT_REQ_ID.fetch_add(1, Ordering::SeqCst)
                );
                recorded_session_ids.push(sid.clone());

                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "sessionId": sid,
                        "configOptions": make_config_options(&current_model, &current_effort),
                    }
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
            }
            "session/set_config_option" => {
                let params = val.get("params");
                let config_id = params
                    .and_then(|p| p.get("configId"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("");
                let value = params
                    .and_then(|p| p.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                let valid = match config_id {
                    "model" => matches!(value, "mock-fast" | "mock-smart"),
                    "effort" => matches!(value, "low" | "medium" | "high"),
                    _ => false,
                };

                if !valid {
                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "error": {
                            "code": -32602,
                            "message": format!("Invalid config option or value: {config_id}={value}")
                        }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                } else {
                    if config_id == "model" {
                        current_model = value.to_string();
                    } else if config_id == "effort" {
                        current_effort = value.to_string();
                    }
                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "configOptions": make_config_options(&current_model, &current_effort),
                        }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                }
            }
            "session/load" => {
                if let Some(params) = val.get("params") {
                    let cwd = params.get("cwd").and_then(|c| c.as_str()).unwrap_or("");
                    assert!(!cwd.is_empty(), "cwd is required in session/load");
                    let cwd_path = std::path::Path::new(cwd);
                    assert!(
                        cwd_path.is_absolute(),
                        "cwd must be absolute in session/load"
                    );

                    if let Some(servers) = params.get("mcpServers").and_then(|s| s.as_array()) {
                        recorded_mcp_servers = servers.clone();
                    }
                }
                let sid = val
                    .get("params")
                    .and_then(|p| p.get("sessionId"))
                    .and_then(|s| s.as_str())
                    .unwrap_or("mock-session-loaded")
                    .to_string();
                recorded_session_ids.push(sid.clone());
                // as the protocol says, the agent replays the conversation before
                // it answers: the client must not show it as a new reply
                let replay = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": sid,
                        "update": {
                            "sessionUpdate": "agent_message_chunk",
                            "content": { "type": "text", "text": "REPLAYED old turn" }
                        }
                    }
                });
                let _ = writeln!(stdout, "{replay}");

                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "configOptions": make_config_options(&current_model, &current_effort),
                    }
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
            }
            "session/prompt" => {
                let session_id = val
                    .get("params")
                    .and_then(|p| p.get("sessionId"))
                    .and_then(|s| s.as_str())
                    .unwrap_or("");

                // Read prompt as ContentBlock array and join the text blocks
                let prompt_blocks_arr = val
                    .get("params")
                    .and_then(|p| p.get("prompt"))
                    .and_then(|p| p.as_array());

                let prompt: String = prompt_blocks_arr
                    .map(|blocks| {
                        blocks
                            .iter()
                            .filter_map(|b| {
                                if b.get("type").and_then(|t| t.as_str()) == Some("text") {
                                    b.get("text").and_then(|t| t.as_str())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("")
                    })
                    .unwrap_or_default();

                let image_count = prompt_blocks_arr
                    .map(|blocks| {
                        blocks
                            .iter()
                            .filter(|b| b.get("type").and_then(|t| t.as_str()) == Some("image"))
                            .count()
                    })
                    .unwrap_or(0);

                // 0. Echo teacher grounding and configuration if requested
                if prompt.contains("[crawrad-teacher]") {
                    // the teacher writes the code as the "Code: E0382" line
                    let code = prompt
                        .lines()
                        .find_map(|l| l.strip_prefix("Code: "))
                        .unwrap_or("none")
                        .trim()
                        .to_string();

                    let explain = if prompt.contains("## Compiler explanation") {
                        "yes"
                    } else {
                        "no"
                    };

                    // the chapter the teacher chose, in its "## Book" section (rustc's own
                    // text may link other chapters)
                    let section = prompt.find("## Book").map_or("", |i| &prompt[i..]);
                    let book = if let Some(idx) = section.find("https://doc.rust-lang.org/book/") {
                        let rest = &section[idx..];
                        let end = rest
                            .find(|c: char| {
                                c.is_whitespace() || c == ')' || c == '>' || c == ']' || c == '"'
                            })
                            .unwrap_or(rest.len());
                        rest[..end].to_string()
                    } else {
                        "none".to_string()
                    };

                    let lang = if let Some(idx) = prompt.find("Answer in ") {
                        let after = &prompt[idx + "Answer in ".len()..];
                        let end = after.find([',', '.']).unwrap_or(after.len());
                        after[..end].trim().to_string()
                    } else {
                        "none".to_string()
                    };

                    let echo = format!(
                        "TEACHER_ECHO\ncode={code}\nexplain={explain}\nbook={book}\nlang={lang}"
                    );

                    let msg = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "agent_message_chunk",
                                "content": {
                                    "type": "text",
                                    "text": echo
                                }
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{msg}");
                    let _ = stdout.flush();

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "stopReason": "end_turn"
                        }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                    continue;
                }

                // 1. Simulate process crash if requested
                if prompt.contains("CRASH") {
                    std::process::exit(101);
                }

                // 1b. Test reply with multiple code blocks and prose between
                if prompt.contains("TEST_THREE_BLOCKS") {
                    let reply_markdown = "Here is an opening explanation.\n```rust\nfn alpha() -> i32 { 1 }\n```\nHere is some connecting prose.\n```rust\nfn beta() -> i32 { 2 }\n```\nHere is another note.\n```toml\n[package]\nname = \"demo\"\n```\nHere is a closing sentence.";
                    let update = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "agent_message_chunk",
                                "content": {
                                    "type": "text",
                                    "text": reply_markdown
                                }
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{update}");
                    let _ = stdout.flush();

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "stopReason": "end_turn"
                        }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                    continue;
                }

                // 2. Simulate hang on cancel: sends initial message chunk then waits for session/cancel notification
                if prompt.contains("HANG_ON_CANCEL") {
                    let update = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "agent_message_chunk",
                                "content": {
                                    "type": "text",
                                    "text": "Starting long task..."
                                }
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{update}");
                    let _ = stdout.flush();

                    for line in lines.by_ref() {
                        let Ok(l) = line else { break };
                        let trimmed = l.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed)
                            && val.get("method").and_then(|m| m.as_str()) == Some("session/cancel")
                        {
                            assert!(
                                val.get("id").is_none(),
                                "session/cancel must be a notification without id"
                            );
                            let resp = serde_json::json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "stopReason": "cancelled"
                                }
                            });
                            let _ = writeln!(stdout, "{resp}");
                            let _ = stdout.flush();
                            break;
                        }
                    }
                    continue;
                }

                // 3. Test status steps scenario: thought -> pause -> tool_call -> pause -> tool_call_update -> pause -> tool_call_update -> pause -> agent_message_chunk -> end_turn
                if prompt.contains("TEST_STATUS_STEPS") {
                    let thought = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "agent_thought_chunk",
                                "content": {
                                    "type": "text",
                                    "text": "Looking for the crate that holds the tests"
                                }
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{thought}");
                    let _ = stdout.flush();
                    std::thread::sleep(std::time::Duration::from_millis(1500));

                    let tc = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "tool_call",
                                "toolCallId": "t1",
                                "title": "Terminal",
                                "kind": "execute",
                                "status": "pending",
                                "rawInput": {}
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{tc}");
                    let _ = stdout.flush();
                    std::thread::sleep(std::time::Duration::from_millis(300));

                    let update1 = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "tool_call_update",
                                "toolCallId": "t1",
                                "title": "cargo test -p app",
                                "status": "in_progress"
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{update1}");
                    let _ = stdout.flush();
                    std::thread::sleep(std::time::Duration::from_millis(2000));

                    let update2 = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "tool_call_update",
                                "toolCallId": "t1",
                                "status": "completed"
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{update2}");
                    let _ = stdout.flush();
                    std::thread::sleep(std::time::Duration::from_millis(1000));

                    let msg = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "agent_message_chunk",
                                "content": {
                                    "type": "text",
                                    "text": "All 42 tests passed in app."
                                }
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{msg}");
                    let _ = stdout.flush();

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "stopReason": "end_turn"
                        }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                    continue;
                }

                // 4. Test reading text file via client request fs/read_text_file
                if prompt.contains("TEST_READ_FILE") {
                    let (path, line, limit) = if let Some(idx) = prompt.find("TEST_READ_FILE") {
                        let after = &prompt[idx + "TEST_READ_FILE".len()..];
                        let parts: Vec<&str> = after.split_whitespace().collect();
                        let p = parts.first().copied().unwrap_or("").trim();
                        let l = parts.get(1).and_then(|s| s.parse::<u64>().ok());
                        let lim = parts.get(2).and_then(|s| s.parse::<u64>().ok());
                        (p, l, lim)
                    } else {
                        ("", None, None)
                    };

                    let req_id = NEXT_REQ_ID.fetch_add(1, Ordering::SeqCst);
                    let mut params = serde_json::json!({
                        "sessionId": session_id,
                        "path": path
                    });
                    if let Some(l) = line {
                        params["line"] = serde_json::json!(l);
                    }
                    if let Some(lim) = limit {
                        params["limit"] = serde_json::json!(lim);
                    }
                    let fs_req = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "method": "fs/read_text_file",
                        "params": params
                    });
                    let _ = writeln!(stdout, "{fs_req}");
                    let _ = stdout.flush();

                    // Wait for client response line directly from the stream
                    if let Some(Ok(resp_line)) = lines.next()
                        && let Ok(resp_val) = serde_json::from_str::<serde_json::Value>(&resp_line)
                    {
                        let content = resp_val
                            .get("result")
                            .and_then(|r| r.get("content"))
                            .and_then(|c| c.as_str())
                            .unwrap_or("<empty>");

                        let update = serde_json::json!({
                            "jsonrpc": "2.0",
                            "method": "session/update",
                            "params": {
                                "sessionId": session_id,
                                "update": {
                                    "sessionUpdate": "agent_message_chunk",
                                    "content": {
                                        "type": "text",
                                        "text": format!("FileContent: {content}")
                                    }
                                }
                            }
                        });
                        let _ = writeln!(stdout, "{update}");
                        let _ = stdout.flush();
                    }

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "stopReason": "end_turn"
                        }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                    continue;
                }

                // 4b. Test writing text file via client request fs/write_text_file
                if prompt.contains("TEST_WRITE_FILE") {
                    let (path, content) = if let Some(idx) = prompt.find("TEST_WRITE_FILE") {
                        let after = &prompt[idx + "TEST_WRITE_FILE".len()..];
                        let parts: Vec<&str> = after.split_whitespace().collect();
                        let p = parts.first().copied().unwrap_or("").trim();
                        let c = parts.get(1).copied().unwrap_or("written_content").trim();
                        (p, c)
                    } else {
                        ("", "written_content")
                    };

                    let req_id = NEXT_REQ_ID.fetch_add(1, Ordering::SeqCst);
                    let fs_req = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "method": "fs/write_text_file",
                        "params": {
                            "sessionId": session_id,
                            "path": path,
                            "content": content
                        }
                    });
                    let _ = writeln!(stdout, "{fs_req}");
                    let _ = stdout.flush();

                    if let Some(Ok(resp_line)) = lines.next()
                        && let Ok(resp_val) = serde_json::from_str::<serde_json::Value>(&resp_line)
                    {
                        let res_is_null_or_empty = resp_val
                            .get("result")
                            .map(|r| {
                                r.is_null() || r.as_object().map(|o| o.is_empty()).unwrap_or(false)
                            })
                            .unwrap_or(false);
                        let status = if res_is_null_or_empty { "OK" } else { "ERR" };
                        let update = serde_json::json!({
                            "jsonrpc": "2.0",
                            "method": "session/update",
                            "params": {
                                "sessionId": session_id,
                                "update": {
                                    "sessionUpdate": "agent_message_chunk",
                                    "content": {
                                        "type": "text",
                                        "text": format!("WriteFile: {status}")
                                    }
                                }
                            }
                        });
                        let _ = writeln!(stdout, "{update}");
                        let _ = stdout.flush();
                    }

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "stopReason": "end_turn"
                        }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                    continue;
                }

                // 5. Test querying MCP servers received in session/new
                if prompt.contains("liste as ferramentas do crawrad-ide")
                    || prompt.contains("mcp_servers")
                {
                    let count = recorded_mcp_servers.len();
                    let names: Vec<String> = recorded_mcp_servers
                        .iter()
                        .filter_map(|s| {
                            s.get("name")
                                .and_then(|n| n.as_str())
                                .map(|n| n.to_string())
                        })
                        .collect();
                    let update = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "agent_message_chunk",
                                "content": {
                                    "type": "text",
                                    "text": format!("Connected MCP servers ({count}): {}", names.join(", "))
                                }
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{update}");
                    let _ = stdout.flush();

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "stopReason": "end_turn"
                        }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                    continue;
                }

                // 6. Test requesting permission via session/request_permission
                if prompt.contains("TEST_REQUEST_PERMISSION") {
                    let tool_name = if let Some(idx) = prompt.find("TEST_REQUEST_PERMISSION") {
                        let after = &prompt[idx + "TEST_REQUEST_PERMISSION".len()..];
                        after
                            .split_whitespace()
                            .next()
                            .unwrap_or("unknown_tool")
                            .trim()
                    } else {
                        "unknown_tool"
                    };

                    let req_id = NEXT_REQ_ID.fetch_add(1, Ordering::SeqCst);
                    let perm_req = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id,
                        "method": "session/request_permission",
                        "params": {
                            "sessionId": session_id,
                            "toolCall": {
                                "name": tool_name,
                                "target": "crates/crawrad-ide/src/main.rs",
                                "arguments": { "sample": true }
                            },
                            "options": [
                                { "optionId": "allow_once", "name": "Allow Once", "kind": "allow_once" },
                                { "optionId": "allow_always", "name": "Always Allow", "kind": "allow_always" },
                                { "optionId": "reject_once", "name": "Reject Once", "kind": "reject_once" },
                                { "optionId": "reject_always", "name": "Never Allow", "kind": "reject_always" }
                            ]
                        }
                    });
                    let _ = writeln!(stdout, "{perm_req}");
                    let _ = stdout.flush();

                    // Wait for client permission response
                    if let Some(Ok(resp_line)) = lines.next()
                        && let Ok(resp_val) = serde_json::from_str::<serde_json::Value>(&resp_line)
                    {
                        let outcome = resp_val
                            .get("result")
                            .and_then(|r| r.get("outcome"))
                            .cloned()
                            .unwrap_or(serde_json::json!({}));
                        let update = serde_json::json!({
                            "jsonrpc": "2.0",
                            "method": "session/update",
                            "params": {
                                "sessionId": session_id,
                                "update": {
                                    "sessionUpdate": "agent_message_chunk",
                                    "content": {
                                        "type": "text",
                                        "text": format!("PermissionOutcome: {}", outcome)
                                    }
                                }
                            }
                        });
                        let _ = writeln!(stdout, "{update}");
                        let _ = stdout.flush();
                    }

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": { "stopReason": "end_turn" }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                    continue;
                }

                // 7. Test two nearly simultaneous permission requests
                if prompt.contains("TEST_TWO_PERMISSIONS") {
                    let req_id1 = NEXT_REQ_ID.fetch_add(1, Ordering::SeqCst);
                    let req_id2 = NEXT_REQ_ID.fetch_add(1, Ordering::SeqCst);

                    let req1 = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id1,
                        "method": "session/request_permission",
                        "params": {
                            "sessionId": session_id,
                            "toolCall": {
                                "name": "write_file_alpha",
                                "target": "crates/crawrad-ide/src/alpha.rs",
                                "arguments": {}
                            },
                            "options": [
                                { "optionId": "allow_once", "name": "Allow Once", "kind": "allow_once" },
                                { "optionId": "reject_once", "name": "Reject Once", "kind": "reject_once" }
                            ]
                        }
                    });
                    let req2 = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": req_id2,
                        "method": "session/request_permission",
                        "params": {
                            "sessionId": session_id,
                            "toolCall": {
                                "name": "write_file_beta",
                                "target": "crates/crawrad-ide/src/beta.rs",
                                "arguments": {}
                            },
                            "options": [
                                { "optionId": "allow_once", "name": "Allow Once", "kind": "allow_once" },
                                { "optionId": "reject_once", "name": "Reject Once", "kind": "reject_once" }
                            ]
                        }
                    });

                    let _ = writeln!(stdout, "{req1}");
                    let _ = writeln!(stdout, "{req2}");
                    let _ = stdout.flush();

                    let mut outcomes = Vec::new();
                    for _ in 0..2 {
                        if let Some(Ok(resp_line)) = lines.next()
                            && let Ok(resp_val) =
                                serde_json::from_str::<serde_json::Value>(&resp_line)
                        {
                            let id_val = resp_val.get("id").and_then(|i| i.as_u64()).unwrap_or(0);
                            let opt_id = resp_val
                                .get("result")
                                .and_then(|r| r.get("outcome"))
                                .and_then(|o| o.get("optionId"))
                                .and_then(|s| s.as_str())
                                .unwrap_or("none");
                            outcomes.push(format!("req_{id_val}:{opt_id}"));
                        }
                    }

                    let update = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "session/update",
                        "params": {
                            "sessionId": session_id,
                            "update": {
                                "sessionUpdate": "agent_message_chunk",
                                "content": {
                                    "type": "text",
                                    "text": format!("TwoPermissionsOutcomes: {}", outcomes.join(";"))
                                }
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{update}");
                    let _ = stdout.flush();

                    let resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": { "stopReason": "end_turn" }
                    });
                    let _ = writeln!(stdout, "{resp}");
                    let _ = stdout.flush();
                    continue;
                }

                // 4. A turn that uses a tool (status line E2E): a thought, a tool
                //    call that runs and completes, then the usual echo reply.
                if prompt.contains("use a tool") {
                    let updates = [
                        serde_json::json!({
                            "sessionUpdate": "agent_thought_chunk",
                            "content": { "type": "text", "text": "Looking for the tests" }
                        }),
                        serde_json::json!({
                            "sessionUpdate": "tool_call",
                            "toolCallId": "t1",
                            "title": "cargo test -p app",
                            "status": "pending"
                        }),
                        serde_json::json!({
                            "sessionUpdate": "tool_call_update",
                            "toolCallId": "t1",
                            "status": "in_progress"
                        }),
                        serde_json::json!({
                            "sessionUpdate": "tool_call_update",
                            "toolCallId": "t1",
                            "status": "completed"
                        }),
                    ];
                    for (i, update) in updates.into_iter().enumerate() {
                        if i == 3 {
                            std::thread::sleep(std::time::Duration::from_millis(1500));
                        }
                        let note = serde_json::json!({
                            "jsonrpc": "2.0",
                            "method": "session/update",
                            "params": { "sessionId": session_id, "update": update }
                        });
                        let _ = writeln!(stdout, "{note}");
                        let _ = stdout.flush();
                    }
                }

                // Standard turn with thinking and streaming updates
                let think = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": session_id,
                        "update": {
                            "sessionUpdate": "agent_thought_chunk",
                            "content": {
                                "type": "text",
                                "text": "Thinking..."
                            }
                        }
                    }
                });
                if !prompt.contains("use a tool") {
                    let _ = writeln!(stdout, "{think}");
                    let _ = stdout.flush();
                }

                let echo_text = if image_count > 0 {
                    format!("Echo on session {session_id}: {prompt} [images: {image_count}]")
                } else {
                    format!("Echo on session {session_id}: {prompt}")
                };

                let delta = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "session/update",
                    "params": {
                        "sessionId": session_id,
                        "update": {
                            "sessionUpdate": "agent_message_chunk",
                            "content": {
                                "type": "text",
                                "text": echo_text
                            }
                        }
                    }
                });
                let _ = writeln!(stdout, "{delta}");
                let _ = stdout.flush();

                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "stopReason": "end_turn"
                    }
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
            }
            "session/cancel" => {
                assert!(
                    val.get("id").is_none(),
                    "session/cancel must be a notification without id"
                );
            }
            "session/shutdown" => {
                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "success": true
                    }
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
                break;
            }
            _ => {}
        }
    }
    if linger {
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}
