//! Tests for the Rust teacher pure module.

use super::*;

#[test]
fn test_error_code_normalization() {
    assert_eq!(error_code(Some("E0382")), Some("E0382".to_string()));
    assert_eq!(error_code(Some("e0502")), Some("E0502".to_string()));
    assert_eq!(error_code(Some("unused_variables")), None);
    assert_eq!(error_code(None), None);
    assert_eq!(error_code(Some("E03821")), None);
}

#[test]
fn test_book_chapter() {
    let ch = book_chapter("E0382");
    assert!(ch.is_some());
    let (title, url) = ch.unwrap();
    assert_eq!(title, "Understanding Ownership");
    assert_eq!(
        url,
        "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"
    );

    assert_eq!(book_chapter("E9999"), None);
    assert_eq!(book_chapter("not_a_code"), None);
}

#[test]
fn test_fix_explain_prompt() {
    let grounding_with_explain = Grounding {
        code: Some("E0382".to_string()),
        message: "use of moved value: `x`".to_string(),
        file: "main.rs".to_string(),
        line: 12,
        snippet: "let y = x;\nlet z = x;".to_string(),
        explain: Some("This error occurs when a value is moved.".to_string()),
    };

    let p = fix_explain_prompt(&grounding_with_explain, "English");
    assert!(p.starts_with("[crawrad-teacher]\n"));
    assert!(p.contains("Code: E0382"));
    assert!(p.contains("Message: use of moved value: `x`"));
    assert!(p.contains("Location: main.rs:12"));
    assert!(p.contains("## Compiler explanation (rustc --explain E0382)"));
    assert!(p.contains("This error occurs when a value is moved."));
    assert!(p.contains("https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"));
    assert!(p.ends_with("Answer in English, but keep code and identifiers in English."));

    let grounding_without_explain = Grounding {
        code: Some("E0382".to_string()),
        message: "use of moved value: `x`".to_string(),
        file: "main.rs".to_string(),
        line: 12,
        snippet: "let y = x;".to_string(),
        explain: None,
    };

    let p2 = fix_explain_prompt(&grounding_without_explain, "Brazilian Portuguese");
    assert!(!p2.contains("## Compiler explanation"));
    assert!(
        p2.ends_with("Answer in Brazilian Portuguese, but keep code and identifiers in English.")
    );
}

#[test]
fn test_rustc_explain_safety_and_execution() {
    assert_eq!(rustc_explain("E0382; rm"), None);
    assert_eq!(rustc_explain("--help"), None);

    let explanation = rustc_explain("E0382");
    assert!(
        explanation.is_some(),
        "rustc --explain E0382 should succeed"
    );
    assert!(
        explanation.unwrap().contains("moved"),
        "E0382 explanation should mention moved"
    );
}

/// **The teacher grounds on the caret line's error first, any severity
/// second, and nothing off the line.**
#[test]
fn pick_on_line_prefers_the_error_on_the_line() {
    // (first line, last line, is error, name)
    let items = [
        (2, 2, false, "warn"),
        (2, 3, true, "err"),
        (5, 5, true, "far"),
    ];
    let span = |d: &(u32, u32, bool, &str)| (d.0, d.1, d.2);
    assert_eq!(pick_on_line(&items, 2, span).map(|d| d.3), Some("err"));
    assert_eq!(
        pick_on_line(&items[..1], 2, span).map(|d| d.3),
        Some("warn")
    );
    assert_eq!(pick_on_line(&items, 3, span).map(|d| d.3), Some("err"));
    assert!(pick_on_line(&items, 4, span).is_none());
}

/// **The excerpt is the lines around the caret, clipped at both ends.**
#[test]
fn snippet_around_clips_at_the_edges() {
    let text = "a\nb\nc\nd\ne";
    assert_eq!(snippet_around(text, 2, 1), "b\nc\nd");
    assert_eq!(snippet_around(text, 0, 3), "a\nb\nc\nd");
    assert_eq!(snippet_around(text, 4, 3), "b\nc\nd\ne");
    assert_eq!(snippet_around(text, 9, 1), "");
}

/// **The teacher's decisions:** no selection, no `/teach`; `/compare` needs
/// exactly two words; no diagnostic, no `/why`; a diagnostic grounds the prompt.
#[test]
fn command_decides_prompt_or_sentence() {
    let say = |r: Reply| matches!(r, Reply::Say(_));
    assert!(say(command("teach", "", "English", false, None)));
    assert!(!say(command("teach", "", "English", true, None)));
    assert!(say(command("compare", "Rc", "English", false, None)));
    assert!(!say(command("compare", "Rc Arc", "English", false, None)));
    assert_eq!(
        command("why", "", "English", false, None),
        Reply::Say(no_diagnostic_message().to_string())
    );
    let g = Grounding {
        code: Some("E0382".into()),
        message: "use of moved value".into(),
        file: "src/main.rs".into(),
        line: 4,
        snippet: "let b = a;".into(),
        explain: None,
    };
    let Reply::Prompt(p) = command("fix-explain", "", "Brazilian Portuguese", false, Some(g))
    else {
        panic!("a grounded fix-explain sends a prompt");
    };
    assert!(p.starts_with("[crawrad-teacher]") && p.contains("Code: E0382"));
    assert!(p.contains("Answer in Brazilian Portuguese"));
}
