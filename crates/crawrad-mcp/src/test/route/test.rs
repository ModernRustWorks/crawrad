//! The bridge expects an answer exactly when the IDE gives one.

use super::{Route, route};

fn expects(line: &str) -> Option<bool> {
    match route(line) {
        Route::Forward { expects_answer, .. } => Some(expects_answer),
        Route::AnswerHere(_) => None,
    }
}

/// **A request is forwarded and answered; a notification is forwarded and
/// not.**
#[test]
fn requests_wait_notifications_do_not() {
    assert_eq!(
        expects(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#),
        Some(true)
    );
    assert_eq!(
        expects(r#"{"jsonrpc":"2.0","id":"a","method":"tools/list"}"#),
        Some(true)
    );
    assert_eq!(
        expects(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#),
        Some(false)
    );
}

/// **`"id": null` is a notification for the IDE, so the bridge must not
/// wait for an answer.** It waited forever before.
#[test]
fn a_null_id_waits_for_nothing() {
    assert_eq!(
        expects(r#"{"jsonrpc":"2.0","id":null,"method":"tools/list"}"#),
        Some(false)
    );
}

/// **What the IDE cannot read is answered here with `-32700`, and never
/// reaches the IDE** — not JSON, a JSON that is not a request, a batch.
#[test]
fn what_the_ide_cannot_read_is_answered_here() {
    for line in [
        r#"{"jsonrpc":"2.0","id":2,"params":{"path":"C:\crawrad"}}"#,
        "{not json",
        r#"[{"jsonrpc":"2.0","id":1,"method":"tools/list"}]"#,
        r#"{"jsonrpc":"2.0","id":3}"#,
    ] {
        match route(line) {
            Route::AnswerHere(v) => {
                assert_eq!(v["error"]["code"], -32700, "{line}");
                assert!(v["id"].is_null(), "{line}");
            }
            other => panic!("{line} was {other:?}"),
        }
    }
}
