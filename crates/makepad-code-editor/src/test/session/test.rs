//! Typing into a session, one key at a time, as the editor receives it.

use crate::{
    decoration::DecorationSet,
    document::CodeDocument,
    history::NewGroup,
    selection::Affinity,
    session::{CodeSession, SelectionMode},
    text::{Position, Text},
};

fn session(text: &str) -> CodeSession {
    CodeSession::new(CodeDocument::new(Text::from(text), DecorationSet::new()))
}

fn at(session: &CodeSession, line_index: usize, byte_index: usize) {
    let p = Position {
        line_index,
        byte_index,
    };
    session.set_selection(p, Affinity::Before, SelectionMode::Simple, NewGroup::Yes);
}

fn type_keys(session: &mut CodeSession, keys: &str) {
    for c in keys.chars() {
        session.insert(Text::from(c.to_string().as_str()));
        session.handle_changes();
    }
}

fn line(session: &CodeSession, i: usize) -> String {
    session.document().as_text().as_lines()[i].clone()
}

/// **Typing a string literal key by key gives exactly that literal.** The
/// opening `"` injects its closing one; typing the closing `"` must step
/// over it — it used to inject another pair (`"Ola";""`).
#[test]
fn typing_a_string_steps_over_the_injected_quote() {
    let mut s = session("\n");
    at(&s, 0, 0);
    type_keys(&mut s, r#"let a = "Ola";"#);
    assert_eq!(line(&s, 0), r#"let a = "Ola";"#);
}

/// **The same for a delimiter:** `(x)` typed is `(x)`.
#[test]
fn typing_parens_steps_over_the_injected_paren() {
    let mut s = session("\n");
    at(&s, 0, 0);
    type_keys(&mut s, "f(x);");
    assert_eq!(line(&s, 0), "f(x);");
}

/// Guarding test: Backspacing inside auto-closed pair preserves overtyping.
#[test]
fn typing_closing_paren_after_backspace_steps_over() {
    let mut s = session("\n");
    at(&s, 0, 0);
    type_keys(&mut s, "area(");
    type_keys(&mut s, "text,");
    s.backspace();
    s.handle_changes();
    type_keys(&mut s, ", n);");
    assert_eq!(line(&s, 0), "area(text, n);");
}

/// **Accepting a completion: the name typed is selected and replaced** —
/// how the shell writes the chosen item (one edit, nothing else touched).
#[test]
fn a_selected_name_is_replaced_by_the_item() {
    let mut s = session("    text.cl();\n");
    at(&s, 0, 9);
    s.move_to(
        Position {
            line_index: 0,
            byte_index: 11,
        },
        Affinity::Before,
        NewGroup::No,
    );
    s.insert(Text::from("clear"));
    s.handle_changes();
    assert_eq!(line(&s, 0), "    text.clear();");
}

/// **A snippet's first stop selected after a multi-line insert.** The
/// insert reaches the session's selections only in `handle_changes`;
/// selecting before it let that later pass push the selection again, past
/// the end of the `}` line — the IDE stopped (`layout.rs` panic).
#[test]
fn selecting_after_a_multiline_insert_waits_for_its_changes() {
    let mut s = session("fn f() {\n    mat\n}\n");
    let start = Position {
        line_index: 1,
        byte_index: 4,
    };
    s.set_selection(
        start,
        Affinity::Before,
        SelectionMode::Simple,
        NewGroup::Yes,
    );
    s.move_to(
        Position {
            line_index: 1,
            byte_index: 7,
        },
        Affinity::Before,
        NewGroup::No,
    );
    s.insert(Text::from("match  {\n    \n}"));
    s.handle_changes();
    let stop = Position {
        line_index: 1,
        byte_index: 10,
    };
    s.set_selection(stop, Affinity::Before, SelectionMode::Simple, NewGroup::Yes);
    s.handle_changes();
    let lines = s.document().as_text().as_lines().to_vec();
    for sel in s.selections().iter() {
        let p = sel.cursor.position;
        assert!(
            p.byte_index <= lines[p.line_index].len(),
            "a caret past its line: {p:?}"
        );
    }
    assert_eq!(s.selections()[0].cursor.position, stop);
    assert_eq!(line(&s, 1), "    match  {");
}

/// **A multi-line insert is indented once:** the `match` body one
/// level under its line, `}` back at the line's level — not the written
/// indentation plus the editor's.
#[test]
fn a_multiline_insert_is_indented_once() {
    let mut s = session("fn f() {\n    mat\n}\n");
    let start = Position {
        line_index: 1,
        byte_index: 4,
    };
    s.set_selection(
        start,
        Affinity::Before,
        SelectionMode::Simple,
        NewGroup::Yes,
    );
    s.move_to(
        Position {
            line_index: 1,
            byte_index: 7,
        },
        Affinity::Before,
        NewGroup::No,
    );
    s.insert(Text::from("match  {\n    \n}"));
    s.handle_changes();
    assert_eq!(line(&s, 1), "    match  {");
    assert_eq!(line(&s, 2), "        ");
    assert_eq!(line(&s, 3), "    }");
}

#[test]
fn set_collapsed_lines_zeroes_height_and_recomputes_y() {
    let s = session("line 0\nline 1\nline 2\nline 3\n");
    let full_height = s.layout().height();
    assert!(s.set_collapsed_lines(&[1]));
    // Calling again with the same collapsed set returns false without recomputing
    assert!(!s.set_collapsed_lines(&[1]));

    let layout = s.layout();
    assert!(!layout.line(0).is_collapsed());
    assert!(layout.line(1).is_collapsed());
    assert!(!layout.line(2).is_collapsed());

    assert_eq!(layout.line(1).height(), 0.0);
    assert_eq!(layout.line(1).scale(), 0.0);

    // Line 0 is at y=0.0 with height 1.0; line 1 is at y=1.0 with height 0.0;
    // so line 2 is at y=1.0 as well.
    assert_eq!(layout.line(0).y(), 0.0);
    assert_eq!(layout.line(1).y(), 1.0);
    assert_eq!(layout.line(2).y(), 1.0);
    assert_eq!(layout.line(3).y(), 2.0);
    // One line of the file collapsed: the page is one line shorter.
    assert_eq!(layout.height(), full_height - 1.0);
    drop(layout);

    // Uncollapsing restores original heights and y positions
    assert!(s.set_collapsed_lines(&[]));
    let layout = s.layout();
    assert_eq!(layout.line(1).height(), 1.0);
    assert_eq!(layout.line(2).y(), 2.0);
    assert_eq!(layout.height(), full_height);
}

#[test]
fn folding_never_restores_or_touches_collapsed_lines() {
    let s = session("fn foo() {\n    let a = 1;\n    let b = 2;\n}\n");
    assert!(s.set_collapsed_lines(&[1]));

    s.fold();
    assert!(!s.update_folds());
    let layout = s.layout();
    assert_eq!(layout.line(1).height(), 0.0);
    drop(layout);

    s.unfold();
    assert!(!s.update_folds());
    let layout = s.layout();
    assert_eq!(layout.line(1).height(), 0.0);
}
