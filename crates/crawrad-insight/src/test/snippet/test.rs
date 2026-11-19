//! Snippets expanded: the text, the tabstops in tab order, no `$` left.

use super::{expand, placed};

fn stops(text: &str, stops: &[(usize, usize)]) -> Vec<String> {
    stops
        .iter()
        .map(|(a, b)| text[*a..*b].to_string())
        .collect()
}

/// rust-analyzer's `soma(…)` (captured 2026-10-04): placeholders in order,
/// `$0` after the call.
#[test]
fn a_call_selects_each_argument_then_ends_after_it() {
    let e = expand("soma(${1:a}, ${2:b})$0", "");
    assert_eq!(e.text, "soma(a, b)");
    assert_eq!(stops(&e.text, &e.stops), vec!["a", "b", ""]);
    assert_eq!(e.stops[2], (10, 10), "$0 after the `)`");
}

/// rust-analyzer's `match` (captured): `$1` empty after `match `, `$0`
/// inside the body, which keeps the indentation of the line it lands on.
#[test]
fn match_keeps_the_line_indentation() {
    let e = expand("match $1 {\n    $0\n}", "    ");
    assert_eq!(e.text, "match  {\n        \n    }");
    assert_eq!(e.stops, vec![(6, 6), (17, 17)]);
    assert!(!e.text.contains('$'));
}

/// Tab order is by number, `$0` last, whatever the order in the text; a
/// repeated number keeps its first place; without `$0` the end is the last
/// stop.
#[test]
fn tab_order_is_by_number_and_zero_is_last() {
    let e = expand("$0 x ${2:b} ${1:a} ${1:a}", "");
    assert_eq!(e.text, " x b a a");
    assert_eq!(stops(&e.text, &e.stops), vec!["a", "b", ""]);
    assert_eq!(e.stops[2], (0, 0));
    let plain = expand("${1:x}", "");
    assert_eq!(plain.stops, vec![(0, 1), (1, 1)], "the end stands for $0");
}

/// Nested placeholders, choices, variables and escapes: none leaves a `$`.
#[test]
fn nested_choices_variables_and_escapes() {
    let e = expand(
        "${1:outer ${2:inner}} ${3|one,two|} $TM_FILENAME ${NAME:dflt} \\$x \\} cost$",
        "",
    );
    assert_eq!(e.text, "outer inner one  dflt $x } cost$");
    assert_eq!(
        stops(&e.text, &e.stops),
        vec!["outer inner", "inner", "one", ""]
    );
}

/// The stops placed on the lines as the editor re-indented them: the
/// `match` body indented one more level than written, `}` one less — the
/// `$0` follows its line, and no stop is past a line's end.
#[test]
fn stops_follow_the_editors_reindent() {
    let e = expand("match $1 {\n    $0\n}", "");
    let doc = |landed: [&str; 3]| -> Vec<String> {
        let mut lines = vec![String::new(); 7];
        lines.extend(landed.iter().map(|l| l.to_string()));
        lines
    };
    // written at (7, 4) of `    mat…`; the editor turned the body into 8
    // spaces and `}` into 4
    let landed = doc(["    match  {", "        ", "    }"]);
    let want = vec![[(7, 10), (7, 10)], [(8, 8), (8, 8)]];
    assert_eq!(placed(&e.text, &e.stops, (7, 4), &landed), want);
    // a body the editor emptied: the stop is clamped to the line
    let emptied = doc(["    match  {", "", "    }"]);
    assert_eq!(
        placed(&e.text, &e.stops, (7, 4), &emptied)[1],
        [(8, 0), (8, 0)]
    );
}
