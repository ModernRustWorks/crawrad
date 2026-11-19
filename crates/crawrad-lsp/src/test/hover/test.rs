//! The hover's contents in the spec's three shapes, made plain.

use super::{Outcome, contents_markdown, diagnostics_at, plain, with_diagnostics};
use serde_json::json;

/// What rust-analyzer sends (captured shape): a `MarkupContent` with the
/// item's path, a ```rust fence with the signature, a rule and the doc.
const RA: &str = "\n```rust\nalloc::string::String\n```\n\n```rust\npub const fn new() -> String\n```\n\n---\n\nCreates a new empty `String`.\n\nSee [`String::with_capacity`](https://doc.rust-lang.org/x) and **more**.";

/// **All three shapes of `contents`** (LSP 3.17 *Hover*): `MarkupContent`,
/// a `MarkedString` (plain or `{language, value}`), and an array of them.
#[test]
fn the_three_shapes_of_contents() {
    let markup = json!({ "contents": { "kind": "markdown", "value": "doc" } });
    assert_eq!(contents_markdown(&markup).as_deref(), Some("doc"));
    let marked = json!({ "contents": "plain" });
    assert_eq!(contents_markdown(&marked).as_deref(), Some("plain"));
    let code = json!({ "contents": { "language": "rust", "value": "fn f()" } });
    assert_eq!(
        contents_markdown(&code).as_deref(),
        Some("```rust\nfn f()\n```")
    );
    let array = json!({ "contents": [{ "language": "rust", "value": "fn f()" }, "text"] });
    assert_eq!(
        contents_markdown(&array).as_deref(),
        Some("```rust\nfn f()\n```\n\ntext")
    );
}

/// **Nothing to say is no card:** null contents, empty text, empty array.
#[test]
fn empty_contents_mean_no_card() {
    for v in [
        json!({}),
        json!({ "contents": "" }),
        json!({ "contents": { "kind": "markdown", "value": "  \n" } }),
        json!({ "contents": [] }),
    ] {
        assert_eq!(contents_markdown(&v), None, "{v}");
    }
}

/// **Plain for the card:** fences go and their code stays, the rule is a
/// line, links keep their text, marks go, blank runs collapse.
#[test]
fn markdown_made_plain() {
    let got = plain(RA);
    let rule = "\u{2500}".repeat(24);
    assert_eq!(
        got,
        format!(
            "alloc::string::String\n\npub const fn new() -> String\n\n{rule}\n\nCreates a new empty String.\n\nSee String::with_capacity and more."
        )
    );
}

/// **Code inside a fence is kept exactly**, marks and all.
#[test]
fn code_in_a_fence_is_kept() {
    let md = "```rust\nlet a = `x`; // **not bold**\n    indented\n```";
    assert_eq!(plain(md), "let a = `x`; // **not bold**\n    indented");
}

/// **The diagnostics under the mouse go on top of the card**, as
/// rust-analyzer 0.3.3065 published them for `total();` under
/// `#[must_use = "the total is the point"]`: the author's line kept, the
/// footer and the hint left out, a range that does not hold the mouse too.
#[test]
fn the_diagnostics_under_the_mouse_lead_the_card() {
    let d = |line, from, to, severity: u8, message: &str| {
        serde_json::from_value::<crate::protocol::Diagnostic>(json!({
            "range": { "start": { "line": line, "character": from },
                       "end": { "line": line, "character": to } },
            "severity": severity, "source": "rustc", "code": "unused_must_use",
            "message": message,
        }))
        .unwrap()
    };
    let published = [
        d(
            50,
            4,
            12,
            2,
            "unused return value of `total` that must be used\nthe total is the point\n`#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default",
        ),
        d(
            50,
            4,
            12,
            4,
            "use `let _ = ...` to ignore the resulting value: `let _ = `",
        ),
        d(49, 4, 9, 2, "use of deprecated function `stale`: use fresh"),
    ];
    let at = crate::protocol::Position {
        line: 50,
        character: 6,
    };
    let written = diagnostics_at(&published, at);
    assert_eq!(
        written,
        "warning: unused return value of `total` that must be used\nthe total is the point\nrustc \u{b7} unused_must_use"
    );
    let card = with_diagnostics(Outcome::Text("pub fn total() -> i32".into()), &written);
    assert_eq!(
        card,
        Outcome::Text(format!("{written}\n\npub fn total() -> i32"))
    );
    assert_eq!(
        with_diagnostics(Outcome::Nothing, &written),
        Outcome::Text(written.clone())
    );
    assert_eq!(with_diagnostics(Outcome::Nothing, ""), Outcome::Nothing);
    let away = crate::protocol::Position {
        line: 52,
        character: 0,
    };
    assert_eq!(diagnostics_at(&published, away), "");
}
