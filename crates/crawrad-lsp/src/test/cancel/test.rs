//! `$/cancelRequest` (LSP 3.17, *Cancellation Support*) against a fake
//! server in the same process, over a pipe: it HOLDS the old request until
//! it is cancelled and answers the new one at once — so the test does not
//! depend on how fast a real server is.

use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::client::{LspClient, LspError};
use crate::complete::CompletionWorker;
use crate::locate::ServerSource;
use crate::protocol::Position;
use crate::transport::{read_message, write_message};

/// What the fake server saw, in order: `"<method> <id>"`.
type Seen = Arc<Mutex<Vec<String>>>;

/// A client wired to a fake server. A completion, highlight or inlay
/// request about line 0 is HELD until its `$/cancelRequest` arrives (then
/// answered `RequestCancelled`); any other line is answered at once.
fn client_and_fake_server() -> (Arc<LspClient>, Seen) {
    let (from_client, client_out) = std::io::pipe().unwrap();
    let (client_in, mut to_client) = std::io::pipe().unwrap();
    let seen: Seen = Arc::default();
    let log = Arc::clone(&seen);
    std::thread::spawn(move || {
        let mut reader = BufReader::new(from_client);
        let mut held: Option<u64> = None;
        while let Ok(msg) = read_message(&mut reader) {
            let v: serde_json::Value = serde_json::from_str(&msg).unwrap();
            let method = v["method"].as_str().unwrap_or("").to_string();
            match method.as_str() {
                "textDocument/completion"
                | "textDocument/documentHighlight"
                | "textDocument/inlayHint" => {
                    let id = v["id"].as_u64().unwrap();
                    log.lock().unwrap().push(format!("{method} {id}"));
                    let p = &v["params"];
                    let line = p["position"]["line"]
                        .as_u64()
                        .or(p["range"]["end"]["line"].as_u64());
                    if line == Some(0) {
                        held = Some(id);
                    } else {
                        let result = match method.as_str() {
                            "textDocument/completion" => serde_json::json!([{ "label": "fresh" }]),
                            "textDocument/documentHighlight" => serde_json::json!([{
                                "range": { "start": { "line": 1, "character": 0 }, "end": { "line": 1, "character": 1 } }
                            }]),
                            _ => serde_json::json!([]),
                        };
                        let answer =
                            serde_json::json!({"jsonrpc": "2.0", "id": id, "result": result});
                        write_message(&mut to_client, &answer.to_string()).unwrap();
                    }
                }
                "$/cancelRequest" => {
                    let id = v["params"]["id"].as_u64().unwrap();
                    log.lock().unwrap().push(format!("{method} {id}"));
                    if held == Some(id) {
                        let answer = serde_json::json!({"jsonrpc": "2.0", "id": id,
                            "error": { "code": -32800, "message": "cancelled" }});
                        write_message(&mut to_client, &answer.to_string()).unwrap();
                        held = None;
                    }
                }
                _ => {}
            }
        }
    });
    let client = LspClient::over_streams(
        None,
        client_out,
        client_in,
        None::<std::io::Empty>,
        ServerSource::Override,
    )
    .unwrap();
    (Arc::new(client), seen)
}

/// Waits until the fake server received a `method` request — the old one
/// is in flight (held) before the newer one is sent. No fixed sleeps.
fn wait_in_flight(seen: &Seen, method: &str) {
    let started = Instant::now();
    while !seen.lock().unwrap().iter().any(|l| l.starts_with(method)) {
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "{method} never reached the server"
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn completion_params(line: u32) -> serde_json::Value {
    serde_json::json!({
        "textDocument": { "uri": "file:///a.rs" },
        "position": { "line": line, "character": 0 }
    })
}

/// **A superseded request is cancelled on the wire and returns at once.**
#[test]
fn a_superseded_request_sends_cancel_request_and_returns() {
    let (client, seen) = client_and_fake_server();
    let started = Instant::now();
    let result = client.send_request_until(
        "textDocument/completion",
        completion_params(0),
        Duration::from_secs(10),
        &mut || started.elapsed() > Duration::from_millis(80),
    );
    assert!(matches!(result, Err(LspError::Cancelled)), "{result:?}");
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "waited for a held request"
    );
    std::thread::sleep(Duration::from_millis(100));
    let log = seen.lock().unwrap().clone();
    let id = log[0].rsplit(' ').next().unwrap().to_string();
    assert_eq!(
        log,
        [
            format!("textDocument/completion {id}"),
            format!("$/cancelRequest {id}")
        ]
    );
    // The server is not stuck: the next request is answered.
    let next = client.send_request(
        "textDocument/completion",
        completion_params(1),
        Duration::from_secs(5),
    );
    assert!(next.is_ok(), "{next:?}");
}

/// **The worker: a newer completion cancels the one in flight**, and only
/// the newer answer reaches the editor.
#[test]
fn the_worker_cancels_the_completion_a_newer_one_replaced() {
    let (client, seen) = client_and_fake_server();
    let worker = CompletionWorker::new();
    let at = |line| Position { line, character: 0 };
    worker.request(
        Arc::clone(&client),
        std::path::Path::new("a.rs"),
        at(0),
        1,
        crate::complete::Trigger::Invoked,
    );
    wait_in_flight(&seen, "textDocument/completion");
    worker.request(
        Arc::clone(&client),
        std::path::Path::new("a.rs"),
        at(1),
        2,
        crate::complete::Trigger::Invoked,
    );
    let started = Instant::now();
    let answer = loop {
        if let Some(a) = worker.take(2) {
            break a;
        }
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "the newer answer never came"
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(
        answer
            .items
            .iter()
            .map(|i| i.label.as_str())
            .collect::<Vec<_>>(),
        ["fresh"]
    );
    let log = seen.lock().unwrap().clone();
    assert!(
        log.iter().any(|l| l.starts_with("$/cancelRequest")),
        "the old one was not cancelled: {log:?}"
    );
    assert!(
        worker.take(1).is_none(),
        "the cancelled answer must not reach the editor"
    );
}

fn cancels_seen(seen: &Seen, method: &str) -> bool {
    let log = seen.lock().unwrap().clone();
    let held: Vec<String> = log
        .iter()
        .filter(|l| l.starts_with(method))
        .map(|l| l.rsplit(' ').next().unwrap().to_string())
        .collect();
    log.iter().any(|l| {
        l.starts_with("$/cancelRequest")
            && held
                .first()
                .is_some_and(|id| l.ends_with(&format!(" {id}")))
    })
}

/// **The occurrences worker cancels the highlight a newer caret replaced.**
#[test]
fn the_highlight_worker_cancels_the_one_a_newer_caret_replaced() {
    let (client, seen) = client_and_fake_server();
    let worker = crate::highlight::HighlightWorker::new();
    let file = std::path::Path::new("a.rs");
    let text = "ab\nab\n";
    worker.request(Arc::clone(&client), file, 1, text, (0, 0));
    wait_in_flight(&seen, "textDocument/documentHighlight");
    worker.request(Arc::clone(&client), file, 2, text, (1, 0));
    let started = Instant::now();
    let placed = loop {
        if let Some(p) = worker.poll(file, 2, text, (1, 0)) {
            break p;
        }
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "the newer highlight never came"
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(placed.len(), 1);
    assert!(
        cancels_seen(&seen, "textDocument/documentHighlight"),
        "{:?}",
        seen.lock().unwrap()
    );
}

/// **The inlay worker cancels the hints of a version a newer one replaced.**
#[test]
fn the_inlay_worker_cancels_the_hints_a_newer_version_replaced() {
    let (client, seen) = client_and_fake_server();
    let worker = crate::inlay::InlayWorker::new();
    let file = std::path::Path::new("a.rs");
    worker.request(Arc::clone(&client), file, 1, "a"); // ends on line 0: held
    wait_in_flight(&seen, "textDocument/inlayHint");
    worker.request(Arc::clone(&client), file, 2, "a\nb");
    let started = Instant::now();
    let answer = loop {
        if let Some(a) = worker.take() {
            break a;
        }
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "the newer hints never came"
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(
        answer.version, 2,
        "only the newer version's answer reaches the editor"
    );
    assert!(
        cancels_seen(&seen, "textDocument/inlayHint"),
        "{:?}",
        seen.lock().unwrap()
    );
}

/// **The request says why it is made** (LSP 3.17 `CompletionContext`):
/// `Ctrl+Space` = 1, a trigger character = 2 with the character, an
/// incomplete list asked again = 3.
#[test]
fn the_completion_context_says_why() {
    use crate::complete::Trigger;
    assert_eq!(
        Trigger::Invoked.context(),
        serde_json::json!({ "triggerKind": 1 })
    );
    assert_eq!(
        Trigger::Character(':').context(),
        serde_json::json!({ "triggerKind": 2, "triggerCharacter": ":" })
    );
    assert_eq!(
        Trigger::Incomplete.context(),
        serde_json::json!({ "triggerKind": 3 })
    );
}

/// **The trigger characters are the server's**, read from `initialize`.
#[test]
fn the_trigger_characters_are_read_from_the_server() {
    use crate::client::completion_triggers_from;
    let caps =
        serde_json::json!({ "completionProvider": { "triggerCharacters": [":", ".", "'", "("] } });
    assert_eq!(completion_triggers_from(Some(&caps)), [":", ".", "'", "("]);
    assert!(completion_triggers_from(Some(&serde_json::json!({}))).is_empty());
    assert!(completion_triggers_from(None).is_empty());
}
