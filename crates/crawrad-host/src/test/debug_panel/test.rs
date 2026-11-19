use std::path::{Path, PathBuf};

use crawrad_dap::controller::ShownVariable;
use crawrad_dap::session::StackFrame;
use crawrad_dap::{Stop, Update};

use super::{Row, pretty_function, rows};
use crate::breakpoints::Breakpoints;
use crate::debug::Debug;

fn stopped() -> Debug {
    let mut debug = Debug::default();
    debug.apply(Update::Stopped(Stop {
        thread_id: 1,
        reason: "breakpoint".into(),
        frames: vec![
            StackFrame {
                id: 1,
                name: "meu_app::form_main_designer::FormMain::btn_ok_click".into(),
                path: Some(PathBuf::from("C:/app/src/form_main.rs")),
                line: 12,
            },
            StackFrame {
                id: 2,
                name: "_$LT$meu_app..form_main_designer..App$u20$as$u20$makepad_draw..match_event..MatchEvent$GT$::handle_actions::ha5155190f5088096".into(),
                path: Some(PathBuf::from("C:/app/src/form_main.designer.rs")),
                line: 681,
            },
        ],
        locals: vec![ShownVariable {
            name: "self".into(),
            value: "FormMain { … }".into(),
            type_name: None,
            reference: 3,
        }],
    }));
    debug
}

#[test]
fn a_stop_lists_its_locals_then_its_stack_then_the_breakpoints() {
    let mut bps = Breakpoints::default();
    bps.set_lines(Path::new("C:/app/src/form_main.rs"), [12]);
    let got = rows(&stopped(), &bps);
    assert_eq!(
        got,
        vec![
            Row::Section("VARIABLES"),
            Row::Variable {
                name: "self".into(),
                value: "FormMain { … }".into(),
                depth: 0,
                reference: 3,
                open: false,
            },
            Row::Section("WATCH"),
            Row::Note(
                "no watches — Add Watch watches the selection or the word under the caret".into()
            ),
            Row::Section("CALL STACK"),
            Row::Frame {
                index: 0,
                function: "…_designer::FormMain::btn_ok_click".into(),
                place: "form_main.rs:12".into(),
                selected: true,
                own: true,
            },
            Row::Frame {
                index: 1,
                function: "…vent::MatchEvent>::handle_actions".into(),
                place: "form_main.designer.rs:681".into(),
                selected: false,
                own: true,
            },
            Row::Section("BREAKPOINTS"),
            Row::Breakpoint {
                file: "form_main.rs".into(),
                line: 12
            },
        ]
    );
}

#[test]
fn with_no_session_each_section_says_why_it_is_empty() {
    let got = rows(&Debug::default(), &Breakpoints::default());
    let sections: Vec<&Row> = got
        .iter()
        .filter(|r| matches!(r, Row::Section(_)))
        .collect();
    assert_eq!(sections.len(), 4);
    let notes = got
        .iter()
        .filter(|r| matches!(r, Row::Note(n) if !n.is_empty()))
        .count();
    assert_eq!(notes, 4, "every empty section explains itself: {got:?}");
}

#[test]
fn the_linker_name_of_a_trait_method_reads_as_rust() {
    assert_eq!(
        pretty_function(
            "_$LT$meu_app..form_main_designer..App$u20$as$u20$makepad_platform..app_main..AppMain$GT$::handle_event::h83b49df4a6a9529d"
        ),
        "<meu_app::form_main_designer::App as makepad_platform::app_main::AppMain>::handle_event"
    );
}

#[test]
fn a_readable_name_is_left_alone_but_loses_its_hash() {
    assert_eq!(
        pretty_function("meu_app::FormMain::btn_ok_click"),
        "meu_app::FormMain::btn_ok_click"
    );
    assert_eq!(
        pretty_function("makepad_draw::match_event::MatchEvent::match_event::h60410c1ebd34f8e6"),
        "makepad_draw::match_event::MatchEvent::match_event"
    );
    // `::h` followed by something that is not the hash stays.
    assert_eq!(pretty_function("app::handle"), "app::handle");
}

#[test]
fn every_row_names_a_template() {
    for row in rows(&stopped(), &Breakpoints::default()) {
        assert!(!row.template().is_empty());
    }
}

#[test]
fn a_long_value_is_one_line_cut_with_an_ellipsis() {
    let form = format!(
        "FormMain {{\n    title: \"x\",\n{}}}",
        "    field: 1,\n".repeat(40)
    );
    let shown = super::preview(&form);
    assert!(!shown.contains('\n'), "{shown}");
    assert!(shown.ends_with('…'), "{shown}");
    assert_eq!(shown.chars().count(), super::PREVIEW_CHARS);
    assert_eq!(super::preview("42"), "42");
}

#[test]
fn a_cut_keeps_the_end_of_the_name() {
    assert_eq!(super::tail("abcdef", 4), "…def");
    assert_eq!(super::tail("abc", 4), "abc");
}

/// A stopped session whose controller takes commands (it answers nothing: the
/// test plays the session's part by applying the updates itself).
fn stopped_with_a_session() -> Debug {
    let mut debug = Debug::default();
    debug.start(crawrad_dap::DebugController::start(
        || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            Err(crawrad_dap::DapError::Closed)
        },
        Box::new(crawrad_dap::AdapterText),
        || {},
    ));
    let stopped = stopped();
    debug.apply(Update::Stopped(stopped.current_stop().unwrap().clone()));
    debug
}

fn child(name: &str, value: &str, reference: i64) -> ShownVariable {
    ShownVariable {
        name: name.into(),
        value: value.into(),
        type_name: None,
        reference,
    }
}

#[test]
fn an_opened_variable_shows_its_children_one_step_in() {
    let mut debug = stopped_with_a_session();
    assert!(debug.toggle_variable(3), "`self` has children: it opens");
    let waiting = rows(&debug, &Breakpoints::default());
    assert_eq!(waiting[2], Row::Note("reading…".into()), "{waiting:?}");
    debug.apply(Update::Children {
        reference: 3,
        children: vec![
            child("btn_ok", "Button { … }", 7),
            child("width", "670.0", 0),
        ],
    });
    let got = rows(&debug, &Breakpoints::default());
    assert_eq!(
        &got[1..4],
        &[
            Row::Variable {
                name: "self".into(),
                value: "FormMain { … }".into(),
                depth: 0,
                reference: 3,
                open: true
            },
            Row::Variable {
                name: "btn_ok".into(),
                value: "Button { … }".into(),
                depth: 1,
                reference: 7,
                open: false
            },
            Row::Variable {
                name: "width".into(),
                value: "670.0".into(),
                depth: 1,
                reference: 0,
                open: false
            },
        ]
    );
    assert!(!debug.toggle_variable(3), "a second click closes it");
    assert_eq!(
        rows(&debug, &Breakpoints::default())[2],
        Row::Section("WATCH")
    );
}

#[test]
fn a_picked_frame_shows_its_own_locals_and_a_late_answer_for_another_is_dropped() {
    let mut debug = stopped_with_a_session();
    let place = debug.select_frame(1).expect("frame 1 has a place");
    assert_eq!(
        place,
        (PathBuf::from("C:/app/src/form_main.designer.rs"), 681)
    );
    assert_eq!(
        rows(&debug, &Breakpoints::default())[1],
        Row::Note("reading the frame…".into())
    );
    // An answer for a frame that is not the picked one changes nothing.
    debug.apply(Update::Locals {
        frame_id: 1,
        locals: vec![child("stale", "1", 0)],
    });
    assert_eq!(
        rows(&debug, &Breakpoints::default())[1],
        Row::Note("reading the frame…".into())
    );
    debug.apply(Update::Locals {
        frame_id: 2,
        locals: vec![child("actions", "[… 1 item]", 0)],
    });
    let got = rows(&debug, &Breakpoints::default());
    assert!(
        matches!(&got[1], Row::Variable { name, .. } if name == "actions"),
        "{got:?}"
    );
    assert!(got.iter().any(|r| matches!(
        r,
        Row::Frame {
            index: 1,
            selected: true,
            ..
        }
    )));
}

#[test]
fn a_new_stop_forgets_the_picked_frame_and_the_open_variables() {
    let mut debug = stopped_with_a_session();
    debug.toggle_variable(3);
    debug.select_frame(1);
    debug.apply(Update::Stopped(stopped().current_stop().unwrap().clone()));
    assert_eq!(debug.selected_frame(), 0);
    assert_eq!(debug.children(3), None);
}

#[test]
fn a_local_the_adapter_cannot_read_has_nothing_to_open() {
    let mut debug = stopped_with_a_session();
    debug.select_frame(1);
    debug.apply(Update::Locals {
        frame_id: 2,
        locals: vec![child("_val", "<error: variable not available>", 9)],
    });
    let got = rows(&debug, &Breakpoints::default());
    assert!(
        matches!(&got[1], Row::Variable { reference: 0, .. }),
        "an error value must not show a ▸: {got:?}"
    );
}

#[test]
fn a_watch_is_asked_at_each_stop_and_shows_its_value() {
    let mut debug = stopped_with_a_session();
    assert!(debug.add_watch("  self.btn_ok.enabled "));
    assert!(
        !debug.add_watch("self.btn_ok.enabled"),
        "the same expression twice is one watch"
    );
    assert!(!debug.add_watch("   "), "an empty expression is no watch");
    let at = |debug: &Debug| {
        rows(debug, &Breakpoints::default())
            .into_iter()
            .find(|r| matches!(r, Row::Watch { .. }))
            .unwrap()
    };
    assert!(matches!(at(&debug), Row::Watch { ref value, .. } if value == "reading…"));
    debug.apply(Update::Watches {
        frame_id: 1,
        values: vec![child("self.btn_ok.enabled", "true", 0)],
    });
    assert_eq!(
        at(&debug),
        Row::Watch {
            index: 0,
            expression: "self.btn_ok.enabled".into(),
            value: "true".into(),
            reference: 0,
            open: false
        }
    );
    // A new stop asks again: the old value is not shown as the new one.
    debug.apply(Update::Stopped(stopped().current_stop().unwrap().clone()));
    assert!(matches!(at(&debug), Row::Watch { ref value, .. } if value == "reading…"));
}

#[test]
fn a_watch_answer_for_another_frame_is_dropped_and_watches_outlive_the_session() {
    let mut debug = stopped_with_a_session();
    debug.add_watch("x");
    debug.apply(Update::Watches {
        frame_id: 2,
        values: vec![child("x", "stale", 0)],
    });
    assert_eq!(
        debug.watch_values(),
        None,
        "frame 2 is not the one the panel shows"
    );
    debug.apply(Update::Terminated { exit_code: Some(0) });
    assert_eq!(debug.watches(), ["x".to_string()]);
    let ended = rows(&debug, &Breakpoints::default());
    assert!(
        ended
            .iter()
            .any(|r| matches!(r, Row::Watch { value, .. } if value.starts_with("not available")))
    );
    assert_eq!(debug.remove_watch(0).as_deref(), Some("x"));
    assert_eq!(debug.remove_watch(0), None);
}

#[test]
fn a_frame_from_a_dependency_the_toolchain_or_the_system_is_not_the_projects() {
    use super::is_own_source;
    assert!(is_own_source(Path::new(
        r"C:\crawrad-wt\rdp\exemplos\meu_app\src\form_main.rs"
    )));
    for foreign in [
        r"C:\Users\User\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\makepad-draw-1.0.0\src\match_event.rs",
        r"C:\Users\User\.cargo\git\checkouts\gimli-123\src\lib.rs",
        r"C:\Users\User\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\std\src\rt.rs",
        "/rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/core/src/ops/function.rs",
        "user32.dll`CallWindowProcW",
        "meu_app.exe`main",
        r"D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl",
    ] {
        assert!(!is_own_source(Path::new(foreign)), "{foreign}");
    }
}

#[test]
fn the_inline_values_are_asked_once_and_written_when_the_right_answer_comes() {
    let mut debug = stopped_with_a_session();
    let file = Path::new("C:/app/src/form_main.rs");
    let mut text = "\n".repeat(11);
    text.push_str("        self.btn_ok.enabled = false;\n");
    assert_eq!(
        debug.inline_values_in(file, &text),
        vec![],
        "nothing until the answer"
    );
    // An answer for another frame is stale.
    debug.apply(Update::Inline {
        frame_id: 2,
        values: vec![child("self.btn_ok.enabled", "true", 0)],
    });
    assert_eq!(debug.inline_values_in(file, &text), vec![]);
    debug.apply(Update::Inline {
        frame_id: 1,
        values: vec![child("self.btn_ok.enabled", "false", 0)],
    });
    assert_eq!(
        debug.inline_values_in(file, &text),
        vec![(11, "    self.btn_ok.enabled = false".to_string())]
    );
    assert!(
        debug
            .inline_values_in(Path::new("C:/app/src/other.rs"), &text)
            .is_empty()
    );
}
