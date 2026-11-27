//! A guardrail that lets the command through is not a guardrail.

use super::{State, check};

const DIALOG_OPEN: State = State {
    modal_open: true,
    build_running: false,
    has_selection: true,
};

const BUILD_RUNNING: State = State {
    modal_open: false,
    build_running: true,
    has_selection: true,
};

const NO_SELECTION: State = State {
    modal_open: false,
    build_running: false,
    has_selection: false,
};

const IDLE_WITH_SELECTION: State = State {
    modal_open: false,
    build_running: false,
    has_selection: true,
};

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
        let r = check(tool, DIALOG_OPEN).unwrap_or_else(|| panic!("{tool} must be refused"));
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
    let worst_case = State {
        modal_open: true,
        build_running: true,
        has_selection: false,
    };
    for tool in [
        "view.get_dialog",
        "view.get_status_bar",
        "designer.list_components",
        "editor.get_buffer_content",
        "component.installed",
        "form.check",
        "build.get_diagnostics",
        "designer.get_selection",
        "view.sizes",
    ] {
        assert_eq!(
            check(tool, worst_case),
            None,
            "{tool} must still be allowed"
        );
    }
}

#[test]
fn with_idle_and_selection_nothing_is_in_the_way() {
    for tool in [
        "component.add",
        "run.start",
        "edit.delete",
        "view.get_dialog",
    ] {
        assert_eq!(check(tool, IDLE_WITH_SELECTION), None);
    }
}

#[test]
fn build_running_refuses_starting_builds_and_runs() {
    for tool in [
        "run.start",
        "build.build",
        "build.check",
        "project.run",
        "project.run_without_debugging",
        "project.run_language",
        "project.build_all",
    ] {
        let r = check(tool, BUILD_RUNNING)
            .unwrap_or_else(|| panic!("{tool} must be refused while build runs"));
        assert_eq!(r.code, "build-running");
        assert!(
            r.fix.contains("run.stop"),
            "fix must name stop tool 'run.stop'"
        );
        assert!(r.message.contains(tool), "message must name offending tool");
    }

    // Commands unrelated to building must not be refused by build_running
    for tool in ["component.add", "run.stop", "view.get_status_bar"] {
        assert_eq!(
            check(tool, BUILD_RUNNING),
            None,
            "{tool} must not be blocked by running build"
        );
    }
}

#[test]
fn no_selection_refuses_commands_that_require_selected_component() {
    for tool in [
        "edit.delete",
        "edit.cut",
        "edit.copy",
        "edit.duplicate",
        "inspector.show_events",
        "inspector.rename",
        "align.same_width",
        "order.tab_earlier",
    ] {
        let r = check(tool, NO_SELECTION)
            .unwrap_or_else(|| panic!("{tool} must be refused when no selection"));
        assert_eq!(r.code, "no-selection");
        assert!(
            r.fix.contains("designer.select"),
            "fix must name selection command"
        );
        assert!(r.message.contains(tool), "message must name offending tool");
    }

    // Commands that create or take explicit target must NOT be refused when nothing is selected
    for tool in [
        "component.add",
        "run.start",
        "designer.select",
        "designer.list_components",
    ] {
        assert_eq!(
            check(tool, NO_SELECTION),
            None,
            "{tool} must not need a prior selection"
        );
    }
}
