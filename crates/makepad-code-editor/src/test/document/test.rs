//! The occurrences of the name under the caret, as the document keeps them.

use crate::{decoration::DecorationSet, document::CodeDocument, text::Text};

fn doc() -> CodeDocument {
    CodeDocument::new(
        Text::from("let total = 1;\ntotal += 2;"),
        DecorationSet::new(),
    )
}

/// **Set once, they are there; set the same again, nothing redraws.**
#[test]
fn the_occurrences_replace_and_report_only_a_change() {
    let doc = doc();
    let found = [(0, 4, 9, true), (1, 0, 5, false)];
    assert!(doc.set_occurrences(&found));
    assert_eq!(doc.occurrences().len(), 2);
    assert!(doc.occurrences()[0].write);
    assert!(
        !doc.set_occurrences(&found),
        "the same answer must not redraw"
    );
    assert!(doc.set_occurrences(&[]));
    assert!(doc.occurrences().is_empty());
}

/// **A place the text does not have is dropped, never drawn.**
#[test]
fn an_occurrence_past_the_text_is_dropped() {
    let doc = doc();
    doc.set_occurrences(&[(0, 4, 99, false), (7, 0, 1, false), (1, 3, 3, false)]);
    assert!(doc.occurrences().is_empty());
}

/// **An edit clears them:** the names moved, and the old places would light
/// the wrong text.
#[test]
fn an_edit_clears_the_occurrences() {
    let doc = doc();
    doc.set_occurrences(&[(0, 4, 9, true)]);
    doc.replace(Text::from("let x = 1;"));
    assert!(doc.occurrences().is_empty());
}

/// **One message per line, the first given wins; a line past the text is
/// dropped; the same messages again redraw nothing.**
#[test]
fn line_messages_keep_one_per_line() {
    let doc = doc();
    let given = [
        (1, "mismatched types".to_string(), 0),
        (1, "unused variable".to_string(), 1),
        (9, "nowhere".to_string(), 0),
    ];
    assert!(doc.set_line_messages(&given));
    let kept: Vec<_> = doc
        .line_messages()
        .iter()
        .map(|m| (m.line, m.text.clone(), m.severity))
        .collect();
    assert_eq!(kept, vec![(1, "mismatched types".to_string(), 0)]);
    assert!(!doc.set_line_messages(&given));
}

/// **The change bars: sorted, one per line, a line past the text dropped;
/// the same set again redraws nothing.**
#[test]
fn line_changes_replace_and_report_only_a_change() {
    use crate::document::LineChange::{Saved, Unsaved};
    let doc = doc();
    let given = [(1, Unsaved), (0, Saved), (1, Saved), (9, Unsaved)];
    assert!(doc.set_line_changes(&given));
    assert_eq!(&*doc.line_changes(), &[(0, Saved), (1, Unsaved)]);
    assert!(
        !doc.set_line_changes(&given),
        "the same set must not redraw"
    );
    assert!(doc.set_line_changes(&[]));
    assert!(doc.line_changes().is_empty());
}

/// **A message follows its line through an edit**, as its squiggle does: an
/// Enter at the start of line 0 pushes line 1's message to line 2.
#[test]
fn a_line_message_follows_its_line() {
    use crate::document::{LineMessage, follow_edits};
    use crate::text::{Change, Drift, Edit, Position};
    let mut messages = vec![
        LineMessage {
            line: 1,
            text: "unused".into(),
            severity: 1,
        },
        LineMessage {
            line: 0,
            text: "above the edit".into(),
            severity: 0,
        },
    ];
    let enter = Edit {
        change: Change::Insert(
            Position {
                line_index: 0,
                byte_index: 0,
            },
            Text::from("\n"),
        ),
        drift: Drift::Before,
    };
    follow_edits(&mut messages, &[enter]);
    let lines: Vec<usize> = messages.iter().map(|m| m.line).collect();
    assert_eq!(lines, vec![2, 1]);
}

/// **The text after a line is its lines joined, cut with `…`;** the
/// compiler's footer is left out, the author's `#[must_use]` text kept.
#[test]
fn a_message_is_cut_to_a_glance() {
    use crate::code_editor::message_line;
    assert_eq!(
        message_line("mismatched types\nexpected `i32`"),
        "mismatched types: expected `i32`"
    );
    // Measured on rustc through rust-analyzer 0.3.3065.
    let must_use = "unused return value of `total` that must be used\nthe total is the point\n\
                    `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default";
    assert_eq!(
        message_line(must_use),
        "unused return value of `total` that must be used: the total is the point"
    );
    let long = "x".repeat(300);
    let cut = message_line(&long);
    assert_eq!(cut.chars().count(), 100);
    assert!(cut.ends_with('…'));
}

/// **A diagnostic paints its line end to end and its message in the
/// severity's colour**: 0 an error, 1 a warning, anything else the rest.
#[test]
fn each_severity_takes_its_own_text_and_band_colour() {
    use crate::code_editor::DiagnosticColors;
    use makepad_widgets::Vec4;
    let c = |v: f32| Vec4 {
        x: v,
        y: v,
        z: v,
        w: 1.0,
    };
    let colors = DiagnosticColors {
        error: c(0.1),
        warning: c(0.2),
        info: c(0.3),
        error_band: c(0.4),
        warning_band: c(0.5),
        info_band: c(0.6),
    };
    assert_eq!(colors.of(0), (c(0.1), c(0.4)));
    assert_eq!(colors.of(1), (c(0.2), c(0.5)));
    assert_eq!(colors.of(2), (c(0.3), c(0.6)));
    assert_eq!(colors.of(7), colors.of(2), "past 2 reads as the rest");
}

/// Code lens rows: placed above their lines, carried by edits, found by y.
mod lens {
    use crate::document::CodeDocument;
    use crate::{
        CodeSession,
        decoration::DecorationSet,
        inlays::BlockInlay,
        selection::Affinity,
        session::SelectionMode,
        text::{Position, Text},
        widgets::BlockWidget,
    };

    #[test]
    fn set_lenses_places_replaces_joins_drops_out_of_range_and_keeps_widget() {
        let doc = CodeDocument::new(
            Text::from("fn main() {\n    println!(\"hello\");\n}\n"),
            DecorationSet::new(),
        );
        doc.0
            .layout
            .borrow_mut()
            .block_inlays
            .push((1, BlockInlay::Widget(BlockWidget { height: 2.0 })));

        assert!(doc.set_lenses(&[
            (0, "4 implementations".into()),
            (0, "Run".into()),
            (0, "Debug".into()),
            (99, "dropped".into()),
        ]));

        let inlays = doc.layout().block_inlays.clone();
        assert_eq!(
            inlays,
            vec![
                (
                    0,
                    BlockInlay::Lens("4 implementations | Run | Debug".into())
                ),
                (1, BlockInlay::Widget(BlockWidget { height: 2.0 })),
            ]
        );

        assert!(doc.set_lenses(&[(0, "1 implementation".into())]));
        let inlays = doc.layout().block_inlays.clone();
        assert_eq!(
            inlays,
            vec![
                (0, BlockInlay::Lens("1 implementation".into())),
                (1, BlockInlay::Widget(BlockWidget { height: 2.0 })),
            ]
        );

        assert!(!doc.set_lenses(&[(0, "1 implementation".into())]));
    }

    #[test]
    fn line_under_lens_shifts_down_lines_above_do_not_move() {
        let doc = CodeDocument::new(Text::from("line 0\nline 1\nline 2\n"), DecorationSet::new());
        let session = CodeSession::new(doc.clone());

        assert_eq!(session.layout().line(0).y(), 0.0);
        assert_eq!(session.layout().line(1).y(), 1.0);
        assert_eq!(session.layout().line(2).y(), 2.0);

        assert!(session.set_lenses(&[(1, "Lens above line 1".into())]));

        assert_eq!(session.layout().line(0).y(), 0.0);
        assert_eq!(session.layout().line(1).y(), 2.0);
        assert_eq!(session.layout().line(2).y(), 3.0);
    }

    #[test]
    fn lens_at_y_finds_lens_band_and_returns_none_on_normal_line() {
        let doc = CodeDocument::new(Text::from("line 0\nline 1\nline 2\n"), DecorationSet::new());
        let session = CodeSession::new(doc.clone());
        session.set_lenses(&[(1, "Lens".into())]);

        assert_eq!(session.lens_at_y(0.5), None);
        assert_eq!(session.lens_at_y(1.0), Some(1));
        assert_eq!(session.lens_at_y(1.5), Some(1));
        assert_eq!(session.lens_at_y(2.0), None);
        assert_eq!(session.lens_at_y(2.5), None);
    }

    #[test]
    fn edit_inserting_newline_above_lens_moves_it_down() {
        let doc = CodeDocument::new(Text::from("line 0\nline 1\nline 2\n"), DecorationSet::new());
        let session = CodeSession::new(doc.clone());
        session.set_lenses(&[(1, "Lens".into())]);

        assert_eq!(
            doc.layout().block_inlays,
            vec![(1, BlockInlay::Lens("Lens".into()))]
        );

        session.set_selection(
            Position {
                line_index: 0,
                byte_index: 0,
            },
            Affinity::Before,
            SelectionMode::Simple,
            crate::history::NewGroup::Yes,
        );
        session.insert(Text::newline());

        assert_eq!(
            doc.layout().block_inlays,
            vec![(2, BlockInlay::Lens("Lens".into()))]
        );
        assert_eq!(session.layout().line(2).y(), 3.0);
        assert_eq!(session.lens_at_y(2.5), Some(2));
    }

    fn lens_lines_after(text: &str, lens_line: usize, delete: (Position, Position)) -> Vec<usize> {
        let doc = CodeDocument::new(Text::from(text), DecorationSet::new());
        let session = CodeSession::new(doc.clone());
        session.set_lenses(&[(lens_line, "Lens".into())]);
        session.set_selection(
            delete.0,
            Affinity::Before,
            SelectionMode::Simple,
            crate::history::NewGroup::Yes,
        );
        session.move_to(delete.1, Affinity::After, crate::history::NewGroup::Yes);
        session.delete();
        doc.layout()
            .block_inlays
            .iter()
            .map(|(line, _)| *line)
            .collect()
    }

    fn at(line_index: usize, byte_index: usize) -> Position {
        Position {
            line_index,
            byte_index,
        }
    }

    #[test]
    fn deleting_lines_drops_the_lenses_of_removed_lines_and_lifts_the_rest() {
        let text = "a0
a1
a2
a3
";
        // whole lines 1-2 gone: the lens of line 3 rises to 1
        assert_eq!(lens_lines_after(text, 3, (at(1, 0), at(3, 0))), vec![1]);
        // the lens of a removed line goes with it
        assert_eq!(
            lens_lines_after(text, 2, (at(1, 0), at(3, 0))),
            Vec::<usize>::new()
        );
        // ...and so does the lens of the first removed line
        assert_eq!(
            lens_lines_after(text, 1, (at(1, 0), at(3, 0))),
            Vec::<usize>::new()
        );
        // a lens past the deleted range rises by the lines removed
        assert_eq!(lens_lines_after(text, 3, (at(1, 0), at(2, 0))), vec![2]);
        // a lens above the deleted range stays
        assert_eq!(lens_lines_after(text, 0, (at(1, 0), at(3, 0))), vec![0]);
        // joining line 2 onto line 1 mid-line: line 1 keeps its lens, line 2's goes
        assert_eq!(lens_lines_after(text, 1, (at(1, 1), at(2, 1))), vec![1]);
        assert_eq!(
            lens_lines_after(text, 2, (at(1, 1), at(2, 1))),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn a_newline_typed_at_the_start_of_a_lens_line_carries_the_lens_down() {
        let doc = CodeDocument::new(
            Text::from(
                "a0
a1
",
            ),
            DecorationSet::new(),
        );
        let session = CodeSession::new(doc.clone());
        session.set_lenses(&[(1, "Lens".into())]);
        session.set_selection(
            at(1, 0),
            Affinity::Before,
            SelectionMode::Simple,
            crate::history::NewGroup::Yes,
        );
        session.insert(Text::newline());
        assert_eq!(
            doc.layout().block_inlays,
            vec![(2, BlockInlay::Lens("Lens".into()))]
        );
        session.set_selection(
            at(2, 1),
            Affinity::Before,
            SelectionMode::Simple,
            crate::history::NewGroup::Yes,
        );
        session.insert(Text::newline());
        assert_eq!(
            doc.layout().block_inlays,
            vec![(2, BlockInlay::Lens("Lens".into()))]
        );
    }
}
