use super::{LINES_ABOVE, expressions, paths, texts};
use crawrad_dap::ShownVariable;

fn value(v: &str) -> ShownVariable {
    ShownVariable {
        name: String::new(),
        value: v.into(),
        type_name: None,
        reference: 0,
    }
}

const HANDLER: &str = "impl FormMain {
    pub fn btn_ok_click(&mut self) {
        // self.secret is a comment
        self.edt_nome_.text = format!(\"Hello, {}!\", self.edt_nome_.text);
        self.btn_ok.enabled = false;
        let total = self.items.len();
        self.lbl.text = \"self.fake\".into();
    }
}";

#[test]
fn each_line_names_the_paths_it_uses_and_never_a_bare_self() {
    let got = paths(HANDLER, 6, &["self", "total"]);
    assert_eq!(
        got,
        vec![
            (3, vec!["self.edt_nome_.text".to_string()]),
            (4, vec!["self.btn_ok.enabled".to_string()]),
            (5, vec!["total".to_string(), "self.items".to_string()]),
            (6, vec!["self.lbl.text".to_string()]),
        ],
        "line 1 is a bare `self`, line 2 a comment, the string on line 6 is not code"
    );
}

#[test]
fn every_path_is_asked_once_and_the_answers_go_back_to_their_lines() {
    let lines = paths(HANDLER, 6, &["self", "total"]);
    let asked = expressions(&lines);
    assert_eq!(
        asked,
        [
            "self.edt_nome_.text",
            "self.btn_ok.enabled",
            "total",
            "self.items",
            "self.lbl.text"
        ]
    );
    let values = [
        value("\"Hello, Edit!\""),
        value("false"),
        value("2"),
        value("<error: no such field>"),
        value("\"x\""),
    ];
    assert_eq!(
        texts(&lines, &asked, &values),
        vec![
            (3, "    self.edt_nome_.text = \"Hello, Edit!\"".to_string()),
            (4, "    self.btn_ok.enabled = false".to_string()),
            (5, "    total = 2".to_string()),
            (6, "    self.lbl.text = \"x\"".to_string()),
        ],
        "an unreadable path is left out"
    );
}

#[test]
fn lines_after_the_stop_and_far_above_it_say_nothing() {
    let mut text = String::new();
    for _ in 0..LINES_ABOVE + 5 {
        text.push_str("let y = x;\n");
    }
    let got = paths(&text, LINES_ABOVE + 2, &["x"]);
    assert_eq!(got.first().map(|(l, _)| *l), Some(2));
    assert_eq!(got.last().map(|(l, _)| *l), Some(LINES_ABOVE + 2));
}

#[test]
fn a_long_value_is_cut() {
    let lines = vec![(0, vec!["x".to_string()])];
    let got = texts(&lines, &["x".to_string()], &[value(&"v".repeat(100))]);
    assert!(got[0].1.ends_with('…'));
}

#[test]
fn the_window_stops_at_the_fn_the_program_stands_in() {
    let text = "impl FormMain {
    pub fn btn_ok_click(&mut self) {
        self.btn_ok.enabled = false;
    }
    pub(crate) fn btn_cancel_click(&mut self) {
        self.btn_cancel.text = x;
    }
}";
    let got = paths(text, 5, &["self", "x"]);
    assert_eq!(
        got,
        vec![(5, vec!["self.btn_cancel.text".to_string(), "x".to_string()])],
        "line 2 belongs to btn_ok_click, which is not running"
    );
}
