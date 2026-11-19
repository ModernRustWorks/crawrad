use std::io::BufReader;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::Duration;

use super::{
    CodeLens, CodeLensWorker, LensCommand, LensKind, at_line, code_lenses_from, json,
    parse_code_lens, rows,
};
use crate::client::{LspClient, answer_server_request};
use crate::locate::ServerSource;
use crate::transport::{read_message, write_message};

#[test]
fn unresolved_lens_parses_without_command_and_keeps_data_for_resolve() {
    let input = serde_json::json!({
        "range": {
            "start": { "line": 5, "character": 0 },
            "end": { "line": 5, "character": 10 }
        },
        "data": {
            "item": 42,
            "extra": ["a", "b"]
        }
    });
    let lens = parse_code_lens(&input).unwrap();
    assert_eq!(lens.command, None);
    assert_eq!(lens.range.start.line, 5);
    assert_eq!(lens.range.end.character, 10);
    assert_eq!(lens.to_json(), input);
}

#[test]
fn resolved_show_references_parses_locations() {
    let loc = |line| {
        serde_json::json!({
            "uri": "file:///workspace/src/lib.rs",
            "range": {
                "start": { "line": line, "character": 0 },
                "end": { "line": line, "character": 4 }
            }
        })
    };
    let input = serde_json::json!({
        "range": {
            "start": { "line": 1, "character": 0 },
            "end": { "line": 1, "character": 8 }
        },
        "command": {
            "title": "4 implementations",
            "command": "rust-analyzer.showReferences",
            "arguments": [
                "file:///workspace/src/lib.rs",
                { "line": 1, "character": 0 },
                [loc(10), loc(20), loc(30), loc(40)]
            ]
        }
    });
    let lens = parse_code_lens(&input).unwrap();
    let cmd = lens.command.unwrap();
    assert_eq!(cmd.title, "4 implementations");
    match cmd.kind {
        LensKind::ShowLocations(locs) => {
            assert_eq!(locs.len(), 4);
            assert_eq!(locs[0].range.start.line, 10);
            assert_eq!(locs[1].range.start.line, 20);
            assert_eq!(locs[2].range.start.line, 30);
            assert_eq!(locs[3].range.start.line, 40);
        }
        _ => panic!("expected ShowLocations"),
    }
}

#[test]
fn command_kinds_are_classified() {
    let make_lens = |cmd_name: &str, args: serde_json::Value| {
        serde_json::json!({
            "range": {
                "start": { "line": 0, "character": 0 },
                "end": { "line": 0, "character": 0 }
            },
            "command": {
                "title": "Test",
                "command": cmd_name,
                "arguments": args
            }
        })
    };

    let run =
        parse_code_lens(&make_lens("rust-analyzer.runSingle", serde_json::json!([]))).unwrap();
    assert_eq!(run.command.unwrap().kind, LensKind::Run);

    let debug = parse_code_lens(&make_lens(
        "rust-analyzer.debugSingle",
        serde_json::json!([]),
    ))
    .unwrap();
    assert_eq!(debug.command.unwrap().kind, LensKind::Run);

    let unknown = parse_code_lens(&make_lens("some.other.command", serde_json::json!([]))).unwrap();
    assert_eq!(unknown.command.unwrap().kind, LensKind::Other);

    let invalid_arg = parse_code_lens(&make_lens(
        "rust-analyzer.showReferences",
        serde_json::json!([
            "file:///a.rs",
            { "line": 0, "character": 0 },
            "not a list of locations"
        ]),
    ))
    .unwrap();
    assert_eq!(invalid_arg.command.unwrap().kind, LensKind::Other);

    let missing_arg = parse_code_lens(&make_lens(
        "rust-analyzer.showReferences",
        serde_json::json!(["file:///a.rs", { "line": 0, "character": 0 }]),
    ))
    .unwrap();
    assert_eq!(missing_arg.command.unwrap().kind, LensKind::Other);
}

#[test]
fn null_list_answer_returns_empty_list() {
    assert_eq!(code_lenses_from(&serde_json::Value::Null), Vec::new());
}

#[test]
fn refresh_request_sets_flag_and_take_clears_it() {
    let (tx, rx) = channel();
    let inlay = AtomicBool::new(false);
    let lens = AtomicBool::new(false);
    let msg = r#"{"jsonrpc":"2.0","id":1,"method":"workspace/codeLens/refresh"}"#;
    assert!(answer_server_request(msg, &tx, &inlay, &lens));
    assert!(lens.load(Ordering::SeqCst));
    assert!(!inlay.load(Ordering::SeqCst));
    let reply: serde_json::Value = serde_json::from_str(&rx.recv().unwrap()).unwrap();
    assert_eq!(
        reply,
        serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": null })
    );
}

#[test]
fn worker_resolves_unresolved_lenses_and_orders_by_range() {
    let (from_client, client_out) = std::io::pipe().unwrap();
    let (client_in, mut to_client) = std::io::pipe().unwrap();

    std::thread::spawn(move || {
        let mut reader = BufReader::new(from_client);
        while let Ok(msg) = read_message(&mut reader) {
            let v: serde_json::Value = serde_json::from_str(&msg).unwrap();
            let method = v["method"].as_str().unwrap_or("");
            let id = v["id"].as_u64().unwrap();
            match method {
                "textDocument/codeLens" => {
                    let result = serde_json::json!([
                        {
                            "range": {
                                "start": { "line": 10, "character": 0 },
                                "end": { "line": 10, "character": 5 }
                            },
                            "data": { "unresolved_id": 1 }
                        },
                        {
                            "range": {
                                "start": { "line": 2, "character": 0 },
                                "end": { "line": 2, "character": 5 }
                            },
                            "command": {
                                "title": "Run",
                                "command": "rust-analyzer.runSingle"
                            }
                        }
                    ]);
                    let answer = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": result
                    });
                    write_message(&mut to_client, &answer.to_string()).unwrap();
                }
                "codeLens/resolve" => {
                    let mut lens = v["params"].clone();
                    lens["command"] = serde_json::json!({
                        "title": "2 references",
                        "command": "rust-analyzer.showReferences",
                        "arguments": [
                            "file:///a.rs",
                            { "line": 10, "character": 0 },
                            [
                                {
                                    "uri": "file:///a.rs",
                                    "range": {
                                        "start": { "line": 12, "character": 0 },
                                        "end": { "line": 12, "character": 5 }
                                    }
                                },
                                {
                                    "uri": "file:///a.rs",
                                    "range": {
                                        "start": { "line": 15, "character": 0 },
                                        "end": { "line": 15, "character": 5 }
                                    }
                                }
                            ]
                        ]
                    });
                    let answer = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": lens
                    });
                    write_message(&mut to_client, &answer.to_string()).unwrap();
                }
                _ => {}
            }
        }
    });

    let client = Arc::new(
        LspClient::over_streams(
            None,
            client_out,
            client_in,
            None::<std::io::Empty>,
            ServerSource::Override,
        )
        .unwrap(),
    );

    let worker = CodeLensWorker::new();
    let (wake_tx, wake_rx) = channel();
    worker.set_waker(move || {
        let _ = wake_tx.send(());
    });

    let path = std::path::Path::new("C:/p/src/lib.rs");
    assert!(worker.shown().is_empty());
    worker.request(Arc::clone(&client), path, 1);

    wake_rx.recv_timeout(Duration::from_secs(5)).unwrap();

    let lenses = worker.poll(Arc::clone(&client), path, 1).unwrap();
    assert_eq!(worker.shown(), lenses);
    assert_eq!(lenses.len(), 2);

    let first = &lenses[0];
    assert_eq!(first.range.start.line, 2);
    assert_eq!(first.command.as_ref().unwrap().title, "Run");
    assert_eq!(first.command.as_ref().unwrap().kind, LensKind::Run);

    let second = &lenses[1];
    assert_eq!(second.range.start.line, 10);
    assert_eq!(second.command.as_ref().unwrap().title, "2 references");
    match &second.command.as_ref().unwrap().kind {
        LensKind::ShowLocations(locs) => {
            assert_eq!(locs.len(), 2);
            assert_eq!(locs[0].range.start.line, 12);
            assert_eq!(locs[1].range.start.line, 15);
        }
        _ => panic!("expected ShowLocations"),
    }
}

/// A lens on `line` whose command is `title` doing `kind`; no command
/// without a title.
fn lens(line: u32, title: Option<&str>, kind: LensKind) -> CodeLens {
    let at = |character| crate::protocol::Position { line, character };
    CodeLens {
        range: crate::protocol::Range {
            start: at(0),
            end: at(5),
        },
        command: title.map(|t| LensCommand {
            title: t.to_string(),
            kind,
        }),
        data: None,
    }
}

fn places(n: usize) -> LensKind {
    let place = crate::protocol::Location {
        uri: "file:///a.rs".into(),
        range: crate::protocol::Range::default(),
    };
    LensKind::ShowLocations(vec![place; n])
}

#[test]
fn rows_are_the_titled_lenses_in_order() {
    let lenses = [
        lens(2, Some("Run"), LensKind::Run),
        lens(5, None, LensKind::Other),
        lens(5, Some("3 references"), places(3)),
        lens(10, Some("Debug"), LensKind::Run),
    ];
    let rows = rows(&lenses);
    let want = [(2, "Run"), (5, "3 references"), (10, "Debug")];
    assert_eq!(rows, want.map(|(l, t)| (l, t.to_string())));
}

#[test]
fn a_click_on_a_row_shows_places_before_anything_else() {
    let lenses = [
        lens(5, Some("Run"), LensKind::Run),
        lens(5, Some("1 reference"), places(1)),
        lens(8, Some("Run"), LensKind::Run),
        lens(8, Some("Doc"), LensKind::Other),
    ];
    assert_eq!(
        at_line(&lenses, 5).map(|c| c.title.as_str()),
        Some("1 reference")
    );
    assert_eq!(at_line(&lenses, 8).map(|c| c.title.as_str()), Some("Run"));
    assert!(at_line(&lenses, 12).is_none());
}

#[test]
fn mcp_reads_line_title_kind_and_how_many_places() {
    let lenses = [
        lens(0, Some("Run"), LensKind::Run),
        lens(4, Some("2 references"), places(2)),
        lens(9, None, LensKind::Other),
    ];
    assert_eq!(
        json(&lenses, &[(4, [1.0, 2.0, 3.0, 4.0])]),
        serde_json::json!([
            { "line": 1, "title": "Run", "kind": "run", "locations": 0, "band": null },
            { "line": 5, "title": "2 references", "kind": "show_locations", "locations": 2,
              "band": [1.0, 2.0, 3.0, 4.0] },
            { "line": 10, "title": null, "kind": "other", "locations": 0, "band": null },
        ])
    );
}

#[test]
fn a_title_loses_the_symbol_the_font_cannot_draw() {
    assert_eq!(super::plain_title("\u{25b6}\u{fe0e} Run"), "Run");
    assert_eq!(super::plain_title("\u{2699}\u{fe0e} Debug"), "Debug");
    assert_eq!(super::plain_title("3 references"), "3 references");
}
