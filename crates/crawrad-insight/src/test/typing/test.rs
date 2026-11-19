//! The list follows what is typed, as VS Code's.

use super::{Opening, typed_trigger};
use super::{
    SignatureEdit, dot_context, in_code, list_top, match_score, opens_after_dot, rank,
    signature_edit, typed_since, word_prefix,
};

fn ranked<'a>(labels: &[&'a str], query: &str) -> Vec<&'a str> {
    rank(labels.iter().copied(), query)
        .into_iter()
        .map(|i| labels[i])
        .collect()
}

/// **The owner's screenshot: `sp` keeps `split`, `splitlines` first (they
/// start with it), then `swapcase`, `strip` (its letters in order) — and
/// drops `count`, `find`.**
#[test]
fn sp_keeps_what_vs_code_keeps() {
    let labels = [
        "capitalize",
        "count",
        "find",
        "split",
        "splitlines",
        "strip",
        "swapcase",
    ];
    assert_eq!(
        ranked(&labels, "sp"),
        ["split", "splitlines", "swapcase", "strip"]
    );
}

/// **`tab` finds `tab_order`; case does not matter; empty keeps all, in order.**
#[test]
fn tab_finds_tab_order() {
    let labels = ["visible", "enabled", "text", "tab_order"];
    assert_eq!(ranked(&labels, "tab"), ["tab_order"]);
    assert_eq!(ranked(&labels, "TAB"), ["tab_order"]);
    assert_eq!(ranked(&labels, ""), labels);
    assert_eq!(match_score("text", "tz"), None);
}

/// **Starting with the query beats containing it, which beats letters in order.**
#[test]
fn prefix_beats_substring_beats_letters_in_order() {
    let labels = ["into_text", "text_color", "t_e_x_t"];
    assert_eq!(
        ranked(&labels, "text"),
        ["text_color", "into_text", "t_e_x_t"]
    );
}

/// **What was typed since the list opened; leaving the name closes it.**
#[test]
fn typed_since_the_list_opened() {
    let line = "        self.btn_ok.ta";
    assert_eq!(typed_since(line, 20), Some("ta"));
    assert_eq!(typed_since(line, 22), Some(""));
    assert_eq!(typed_since("self.btn_ok.ta ", 12), None, "a space closes");
    assert_eq!(typed_since("self.btn_ok.ta(", 12), None, "a paren closes");
    assert_eq!(
        typed_since("self.btn_ok", 12),
        None,
        "backspace past the dot closes"
    );
}

/// **A `.` after a name or a call opens the list; after a number or a dot, not.**
#[test]
fn a_dot_opens_the_list_after_a_name() {
    assert!(opens_after_dot("        self."));
    assert!(opens_after_dot("self.btn_ok."));
    assert!(opens_after_dot("prompt.lower()."));
    assert!(!opens_after_dot("let x = 0."));
    assert!(!opens_after_dot("for i in 0.."));
    assert!(!opens_after_dot("self.btn"));
}

/// **The context before the caret** (moved from the shell, with its cases).
#[test]
fn dot_context_and_word_prefix() {
    assert_eq!(word_prefix("let x = St", 10), "St");
    assert_eq!(word_prefix("foo.bar", 7), "bar");
    assert_eq!(word_prefix("let valor = ", 12), "");
    assert_eq!(word_prefix("", 0), "");
    assert_eq!(
        dot_context("    self.", 9),
        (Some("self".into()), String::new(), 9)
    );
    assert_eq!(
        dot_context("    self.edt_", 13),
        (Some("self".into()), "edt_".into(), 9)
    );
    assert_eq!(
        dot_context("    self. edt_", 14),
        (Some("self".into()), "edt_".into(), 9)
    );
    assert_eq!(dot_context("let x = St", 10), (None, "St".into(), 8));
}

/// **The owner's screenshot: `"Cancel".` is a member access too** — the
/// list must wait for rust-analyzer (`into`, `to_string`…), not show the
/// catalogue's classes; and typing the `.` opens it by itself.
#[test]
fn a_dot_after_a_literal_or_a_call_is_a_member_access() {
    let line = r#"        self.btn_cancel.text = "Cancel"."#;
    let (base, prefix, from) = dot_context(line, line.len());
    assert_eq!(base.as_deref(), Some("\""));
    assert_eq!((prefix.as_str(), from), ("", line.len()));
    assert!(opens_after_dot(line));
    for line in ["x = list[0].", "let v = parse()?.", "c = 'a'."] {
        assert!(dot_context(line, line.len()).0.is_some(), "{line}");
        assert!(opens_after_dot(line), "{line}");
    }
    assert!(
        !opens_after_dot("let x = 0."),
        "a number still does not open it"
    );
}

/// **Inside a class, the server's order stands** — the exact match first.
#[test]
fn the_servers_order_stands_inside_a_class() {
    assert_eq!(
        ranked(&["leak", "len", "lex"], "le"),
        ["leak", "len", "lex"]
    );
    // ...except the exact match, which comes first (VS Code does the same).
    assert_eq!(ranked(&["leak", "len", "le"], "le"), ["le", "leak", "len"]);
}

/// **A dot in a comment or inside a string does not open the list.**
#[test]
fn prose_dots_do_not_open_the_list() {
    assert!(!opens_after_dot("    // fim."));
    assert!(!opens_after_dot(r#"    let a = "Ola."#));
    assert!(!opens_after_dot(r#"    let a = "say \"hi\" now."#));
    assert!(opens_after_dot(r#"    let a = "Ola".to_string()."#));
    assert!(opens_after_dot("    let u = \"http://x\"."));
    assert!(in_code("let a = 1;"));
    assert!(!in_code("let a = 1; // note"));
}

/// **The wheel scrolls the list and leaves the selection; the keys pull the
/// window to the selection.** 95 items, 8 rows: `into` at 74 is reachable
/// with the wheel.
#[test]
fn the_wheel_scrolls_and_the_keys_follow_the_selection() {
    assert_eq!(list_top(0, 0, false, 3, 95), 3, "wheel down three rows");
    assert_eq!(list_top(3, 0, false, -5, 95), 0, "not above the first");
    assert_eq!(list_top(80, 0, false, 30, 95), 87, "not past the last page");
    assert_eq!(
        list_top(0, 74, false, 72, 95),
        72,
        "into, at 74, is on screen"
    );
    assert_eq!(
        list_top(0, 9, true, 0, 95),
        2,
        "Down past the page pulls it"
    );
    assert_eq!(list_top(40, 3, true, 0, 95), 3, "Up above it pulls it back");
    assert_eq!(list_top(5, 0, false, 0, 4), 0, "a short list never scrolls");
}

/// **The server's trigger characters open the list (rust-analyzer: `: . ' (`)**
/// — `.` and `::` at once, `'` and `(` quietly, `:` alone never, nothing in
/// a comment or a string, nothing the server did not list.
#[test]
fn the_servers_trigger_characters_open_the_list() {
    let ra: Vec<String> = [":", ".", "'", "("].iter().map(|s| s.to_string()).collect();
    assert_eq!(typed_trigger("    std::", &ra), Some((':', Opening::Loud)));
    assert_eq!(
        typed_trigger("    let v = Vec::", &ra),
        Some((':', Opening::Loud))
    );
    assert_eq!(typed_trigger("    s.", &ra), Some(('.', Opening::Loud)));
    assert_eq!(
        typed_trigger("    fn f<'", &ra),
        Some(('\'', Opening::Quiet))
    );
    assert_eq!(
        typed_trigger("    #[derive(", &ra),
        Some(('(', Opening::Quiet))
    );
    assert_eq!(typed_trigger("    let x: ", &ra), None, "a space");
    assert_eq!(
        typed_trigger("    let x:", &ra),
        None,
        "a single colon is a type"
    );
    assert_eq!(typed_trigger("    let x = 0.", &ra), None, "a number");
    assert_eq!(typed_trigger("    // see std::", &ra), None, "a comment");
    assert_eq!(typed_trigger(r#"    let s = "a::"#, &ra), None, "a string");
    let only_dot = vec![".".to_string()];
    assert_eq!(
        typed_trigger("    std::", &only_dot),
        None,
        "the server did not list ':'"
    );
}

/// **The parameter card:** `(` and `,` typed in code ask; the same
/// characters in a string or a comment do not; any other edit asks again
/// only while a card is up.
#[test]
fn the_parameter_card_follows_the_edit() {
    let ra: Vec<String> = ["(", ",", "<"].iter().map(|s| s.to_string()).collect();
    assert_eq!(
        signature_edit("    soma(", &ra, false),
        Some(SignatureEdit::Typed('('))
    );
    assert_eq!(
        signature_edit("    soma(1,", &ra, true),
        Some(SignatureEdit::Typed(','))
    );
    assert_eq!(signature_edit("    let s = \"a,", &ra, false), None);
    assert_eq!(signature_edit("    // f(", &ra, false), None);
    assert_eq!(
        signature_edit("    soma(1", &ra, true),
        Some(SignatureEdit::Follow),
        "a deleted comma: ask again, the mark moves back"
    );
    assert_eq!(signature_edit("    soma(1)", &ra, false), None);
    assert_eq!(
        signature_edit("    soma(1)", &ra, true),
        Some(SignatureEdit::Follow)
    );
    assert_eq!(
        signature_edit("    soma(", &[], false),
        None,
        "a server that lists none"
    );
}
