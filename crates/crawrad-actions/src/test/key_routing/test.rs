//! Tests for key routing and view affinity with `Front`.

use crate::key_routing::{Comando, Front, blocked, serves_view};
use crawrad_designer::ArrowNudge;

#[test]
fn serves_view_table_over_front_and_actions() {
    let expected_design = [
        ("edit.delete", true),
        ("edit.duplicate", true),
        ("edit.copy", true),
        ("edit.select_all", true),
        ("edit.undo", true),
        ("editor.copy", false),
    ];
    for (id, expected) in expected_design {
        assert_eq!(
            serves_view(id, Front::Design),
            expected,
            "serves_view({id}, Front::Design)"
        );
    }

    let expected_text = [
        ("edit.delete", false),
        ("edit.duplicate", false),
        ("edit.copy", false),
        ("edit.select_all", false),
        ("edit.undo", true),
        ("editor.copy", true),
    ];
    for (id, expected) in expected_text {
        assert_eq!(
            serves_view(id, Front::Text),
            expected,
            "serves_view({id}, Front::Text)"
        );
    }

    let expected_other = [
        ("edit.delete", false),
        ("edit.duplicate", false),
        ("edit.copy", false),
        ("edit.select_all", false),
        ("edit.undo", true),
        ("editor.copy", false),
    ];
    for (id, expected) in expected_other {
        assert_eq!(
            serves_view(id, Front::Other),
            expected,
            "serves_view({id}, Front::Other)"
        );
    }
}

#[test]
fn front_of_flags() {
    assert_eq!(Front::of(true, false), Front::Design);
    assert_eq!(Front::of(false, true), Front::Text);
    assert_eq!(Front::of(false, false), Front::Other);
}

#[test]
fn designer_gestures_only_act_with_the_canvas_in_front() {
    let arrow = Comando::Seta {
        dx: 1.0,
        dy: 0.0,
        modo: ArrowNudge::Move,
    };
    for cmd in [arrow, Comando::Cancelar] {
        assert!(!blocked(&cmd, false, Front::Design), "{cmd:?} on Design");
        assert!(
            blocked(&cmd, false, Front::Text),
            "{cmd:?} on a text editor"
        );
        assert!(
            blocked(&cmd, false, Front::Other),
            "{cmd:?} on History/Sizes/Translations/Welcome"
        );
    }
    // The bug report: Delete with History in front must not reach the hidden form.
    assert!(blocked(&Comando::Acao("edit.delete"), false, Front::Other));
    assert!(!blocked(
        &Comando::Acao("edit.delete"),
        false,
        Front::Design
    ));
    assert!(!blocked(&Comando::Acao("edit.undo"), false, Front::Other));
}
