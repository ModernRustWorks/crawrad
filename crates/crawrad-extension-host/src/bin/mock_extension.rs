#![forbid(unsafe_code)]
//! A mock extension subprocess for integration testing IPC stdio JSON-RPC.

use std::io::{BufRead, BufReader, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        let Ok(l) = line else { break };
        let trimmed = l.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) else {
            continue;
        };

        let method = val.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let id = val.get("id").and_then(|i| i.as_u64());

        match method {
            "extension/initialize" => {
                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "success": true,
                        "commands": ["mock.action", "mock.crash"]
                    }
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
            }
            "extension/ping" => {
                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": "pong"
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
            }
            "extension/executeCommand" => {
                let cmd = val
                    .get("params")
                    .and_then(|p| p.get("command"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("");

                if cmd == "mock.crash" {
                    // Simulate intentional crash with exit code 101 to test fault isolation
                    std::process::exit(101);
                }

                // Paint the line marks the arguments describe.
                if cmd == "mock.marks" {
                    let args = val
                        .pointer("/params/arguments")
                        .cloned()
                        .unwrap_or_default();
                    let marks = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "editor/setLineMarks",
                        "params": {
                            "file": args.get("file").cloned().unwrap_or_default(),
                            "marks": args.get("marks").cloned().unwrap_or_else(|| serde_json::json!([]))
                        }
                    });
                    let _ = writeln!(stdout, "{marks}");
                    let _ = stdout.flush();
                }

                // Fill the view the arguments name with their tree.
                if cmd == "mock.tree" || cmd == "mock2.tree" {
                    let args = val
                        .pointer("/params/arguments")
                        .cloned()
                        .unwrap_or_default();
                    let tree = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "view/setTree",
                        "params": {
                            "view_id": args.get("view_id").cloned().unwrap_or_default(),
                            "items": args.get("items").cloned().unwrap_or_else(|| serde_json::json!([]))
                        }
                    });
                    let _ = writeln!(stdout, "{tree}");
                    let _ = stdout.flush();
                }

                if cmd == "mock.showCard" {
                    let render = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "ui/renderCard",
                        "params": {
                            "card_id": "mock.panel",
                            "root": {
                                "id": "root",
                                "type": "container",
                                "layout": "vertical",
                                "children": [
                                    { "id": "lbl_title", "type": "label", "text": "Mock Card Title" },
                                    { "id": "btn_count", "type": "button", "text": "Click me: 0" }
                                ]
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{render}");
                    let _ = stdout.flush();
                }

                // Send an async notification to the IDE
                let notif = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "window/showMessage",
                    "params": {
                        "level": "info",
                        "message": format!("Mock executed {cmd}")
                    }
                });
                let _ = writeln!(stdout, "{notif}");
                let _ = stdout.flush();

                // Respond with structured result
                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "success": true,
                        "result": {
                            "executed": cmd,
                            "status": "ok"
                        }
                    }
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
            }
            "editor/lineMarkClicked" => {
                let mark = val
                    .pointer("/params/id")
                    .and_then(|i| i.as_str())
                    .unwrap_or("");
                let said = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "window/showMessage",
                    "params": { "level": "info", "message": format!("Mock mark clicked: {mark}") }
                });
                let _ = writeln!(stdout, "{said}");
                let _ = stdout.flush();
            }
            "view/itemClicked" => {
                let item = val
                    .pointer("/params/item_id")
                    .and_then(|i| i.as_str())
                    .unwrap_or("");
                let said = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "window/showMessage",
                    "params": { "level": "info", "message": format!("Mock item clicked: {item}") }
                });
                let _ = writeln!(stdout, "{said}");
                let _ = stdout.flush();
            }
            "ui/event" => {
                let card_id = val
                    .get("params")
                    .and_then(|p| p.get("card_id"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("");
                let element_id = val
                    .get("params")
                    .and_then(|p| p.get("element_id"))
                    .and_then(|e| e.as_str())
                    .unwrap_or("");

                if element_id == "btn_count" {
                    let update = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "ui/updateElement",
                        "params": {
                            "card_id": card_id,
                            "element_id": "btn_count",
                            "patch": {
                                "text": "Clicked 1 time"
                            }
                        }
                    });
                    let _ = writeln!(stdout, "{update}");
                    let _ = stdout.flush();
                } else if element_id == "btn_close" {
                    let close = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "ui/closeCard",
                        "params": {
                            "card_id": card_id
                        }
                    });
                    let _ = writeln!(stdout, "{close}");
                    let _ = stdout.flush();
                }
            }
            "extension/shutdown" => {
                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "success": true }
                });
                let _ = writeln!(stdout, "{resp}");
                let _ = stdout.flush();
                break;
            }
            _ => {}
        }
    }
}
