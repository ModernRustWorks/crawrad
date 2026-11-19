#![forbid(unsafe_code)]
//! `crawrad-todo`: the TODO extension's process. It speaks JSON-RPC on stdio
//! like any extension; every second and a half it reads the project again
//! and, when the tagged comments changed, sends the marks and the tree.

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crawrad_ext_todo::{Todo, list, marks, scan_project, tree};
use crawrad_note::note;
use serde_json::{Value, json};

type Out = Arc<Mutex<std::io::Stdout>>;

#[note("bin-crawrad-todo-send")]
fn send(out: &Out, message: &Value) {
    if let Ok(mut o) = out.lock() {
        let _ = writeln!(o, "{message}");
        let _ = o.flush();
    }
}

fn notify(out: &Out, method: &str, params: Value) {
    send(
        out,
        &json!({ "jsonrpc": "2.0", "method": method, "params": params }),
    );
}

fn answer(out: &Out, id: Option<u64>, result: Value) {
    send(
        out,
        &json!({ "jsonrpc": "2.0", "id": id, "result": result }),
    );
}

#[note("bin-crawrad-todo-publish")]
fn publish(out: &Out, root: &std::path::Path, now: &[Todo], before: &[Todo]) {
    for params in marks(root, now, before) {
        notify(
            out,
            "editor/setLineMarks",
            serde_json::to_value(params).unwrap_or_default(),
        );
    }
    notify(
        out,
        "view/setTree",
        serde_json::to_value(tree(now)).unwrap_or_default(),
    );
}

#[note("bin-crawrad-todo-watch")]
fn watch(out: Out, root: PathBuf, seen: Arc<Mutex<Vec<Todo>>>) {
    std::thread::spawn(move || {
        loop {
            let now = scan_project(&root);
            if let Ok(mut before) = seen.lock()
                && *before != now
            {
                publish(&out, &root, &now, &before);
                *before = now;
            }
            std::thread::sleep(Duration::from_millis(1500));
        }
    });
}

fn main() {
    let out: Out = Arc::new(Mutex::new(std::io::stdout()));
    let seen: Arc<Mutex<Vec<Todo>>> = Arc::default();
    let mut root: Option<PathBuf> = None;
    for line in BufReader::new(std::io::stdin().lock()).lines() {
        let Ok(line) = line else { break };
        let Ok(val) = serde_json::from_str::<Value>(line.trim()) else {
            continue;
        };
        let id = val.get("id").and_then(Value::as_u64);
        match val.get("method").and_then(Value::as_str).unwrap_or("") {
            "extension/initialize" => {
                let ws = val
                    .pointer("/params/workspace_path")
                    .and_then(Value::as_str);
                root = ws.map(PathBuf::from);
                answer(
                    &out,
                    id,
                    json!({ "success": true, "commands": ["todos.list"] }),
                );
                if let Some(r) = &root {
                    watch(out.clone(), r.clone(), seen.clone());
                }
            }
            "extension/ping" => answer(&out, id, json!("pong")),
            "extension/executeCommand" => {
                let tagged = root.as_deref().map(scan_project).unwrap_or_default();
                answer(
                    &out,
                    id,
                    json!({ "success": true, "result": list(&tagged) }),
                );
            }
            // a mark's icon: what the comment says, on the status bar
            "editor/lineMarkClicked" => {
                let mark = val
                    .pointer("/params/id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let found = seen.lock().ok().and_then(|s| {
                    let t = s
                        .iter()
                        .find(|t| format!("{}:{}", t.file, t.line) == mark)?;
                    Some(format!("{}: {}", t.kind.to_uppercase(), t.text))
                });
                if let Some(said) = found {
                    notify(
                        &out,
                        "window/showMessage",
                        json!({ "level": "info", "message": said }),
                    );
                }
            }
            "extension/shutdown" => {
                answer(&out, id, json!({ "success": true }));
                break;
            }
            _ => {}
        }
    }
}
