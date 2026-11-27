//! A guardrail that lets the command through is not a guardrail.

use super::{State, check};

const ABERTO: State = State { modal_open: true };
const FECHADO: State = State { modal_open: false };

#[test]
fn a_dialog_open_refuses_anything_that_changes_the_ide() {
    for tool in [
        "component.add",
        "component.set_property",
        "editor.apply_edit",
        "designer.wire_event",
        "run.start",
        "build.build",
    ] {
        let r = check(tool, ABERTO).unwrap_or_else(|| panic!("{tool} must be refused"));
        assert_eq!(r.code, "dialog-open");
        assert!(
            r.fix.contains("Close the dialog"),
            "the fix must be actionable"
        );
        assert!(
            r.message.contains(tool),
            "the message must name the command"
        );
    }
}

#[test]
fn looking_is_always_allowed_or_the_caller_goes_blind() {
    for tool in [
        "view.get_dialog",
        "view.get_status_bar",
        "designer.list_components",
        "editor.get_buffer_content",
        "component.installed",
        "form.check",
    ] {
        assert_eq!(check(tool, ABERTO), None, "{tool} must still be allowed");
    }
}

#[test]
fn with_no_dialog_nothing_is_in_the_way() {
    for tool in ["component.add", "run.start", "view.get_dialog"] {
        assert_eq!(check(tool, FECHADO), None);
    }
}
