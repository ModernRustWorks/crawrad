//! The server's items by name, as rust-analyzer 1.95 answered them.

use super::{Symbol, parse};
use std::path::PathBuf;

/// **A method keeps its type; a module item has none; `WorkspaceSymbol`
/// without a range lands on line 0** — the three shapes measured.
#[test]
fn the_answer_becomes_items_with_their_type() {
    let uri = crate::path_to_uri(&PathBuf::from("C:/demo/src/lib.rs"));
    let answer = serde_json::json!([
        { "name": "fetch", "kind": 12, "containerName": "RegistryClient",
          "location": { "uri": uri, "range": { "start": { "line": 3, "character": 15 },
                                               "end": { "line": 3, "character": 20 } } } },
        { "name": "fetch_all", "kind": 12, "containerName": "",
          "location": { "uri": uri, "range": { "start": { "line": 9, "character": 7 },
                                               "end": { "line": 9, "character": 16 } } } },
        { "name": "NotFound", "kind": 22, "containerName": "RegistryError",
          "location": { "uri": uri } },
        { "kind": 12, "location": { "uri": uri } },
    ]);
    let found = parse(answer);
    let file = crate::uri_to_path(&uri).expect("a file uri");
    assert_eq!(
        found,
        vec![
            Symbol {
                name: "fetch".into(),
                container: Some("RegistryClient".into()),
                kind: 12,
                file: file.clone(),
                line: 3
            },
            Symbol {
                name: "fetch_all".into(),
                container: None,
                kind: 12,
                file: file.clone(),
                line: 9
            },
            Symbol {
                name: "NotFound".into(),
                container: Some("RegistryError".into()),
                kind: 22,
                file,
                line: 0
            },
        ]
    );
}

/// **No answer is no items**, not a failure.
#[test]
fn a_null_answer_is_no_items() {
    assert!(parse(serde_json::Value::Null).is_empty());
}
