//! Tests for the Problems panel model.

use super::{Diagnostic, DiagnosticLocation, Filter, Problems, Row, Severity};
use std::path::{Path, PathBuf};
use std::time::Duration;

const MISMATCH: &str = "error[E0308]: mismatched types
 --> src/main.rs:8:20
  |
8 |     let y = add(x, \"3\");
  |             ---    ^^^ expected `i32`, found `&str`
  |             |
  |             arguments to this function are incorrect";

const UNUSED: &str = "warning: unused variable: `sum`
 --> src/main.rs:2:9
  |
2 |     let sum = a + b;
  |         ^^^ help: if this is intentional, prefix it with an underscore: `_sum`
  |
  = note: `#[warn(unused_variables)]` on by default";

fn diagnostic(severity: Severity, message: &str, line: usize, rendered: &str) -> Diagnostic {
    Diagnostic {
        severity,
        message: message.into(),
        location: Some(DiagnosticLocation {
            file: PathBuf::from("src/main.rs"),
            line,
            column: if line == 8 { 20 } else { 9 },
        }),
        rendered: rendered.into(),
    }
}

/// The mockup's build: the warning arrives first, the error second.
fn sample() -> Problems {
    let mut problems = Problems::default();
    problems.populate(
        &[
            diagnostic(Severity::Warning, "unused variable: `sum`", 2, UNUSED),
            diagnostic(Severity::Error, "mismatched types", 8, MISMATCH),
        ],
        Some(Path::new("C:/work/dbg_console")),
    );
    problems
}

#[test]
fn rows_are_flat_with_the_error_first_as_the_mockup_draws_them() {
    let rows = sample().rows();
    assert_eq!(
        rows,
        vec![
            Row::Problem {
                index: 0,
                severity: Severity::Error,
                message: "mismatched types: expected i32, found &str".into(),
                place: "src/main.rs:8:20".into(),
                code: "E0308".into(),
            },
            Row::Problem {
                index: 1,
                severity: Severity::Warning,
                message: "unused variable: sum".into(),
                place: "src/main.rs:2:9".into(),
                code: "unused_variables".into(),
            },
        ]
    );
    assert_eq!(rows[0].template(), "ProblemError");
    assert_eq!(rows[1].template(), "ProblemWarning");
}

#[test]
fn summary_and_stop_text_count_both_kinds() {
    let problems = sample();
    assert_eq!(problems.summary(), "1 error, 1 warning");
    assert_eq!(
        problems.stop_text(true, Duration::from_millis(3400)),
        "Run stopped: the build failed \u{2014} 1 error, 1 warning \u{00b7} 3.4 s"
    );
    assert!(
        problems
            .stop_text(false, Duration::ZERO)
            .starts_with("Build failed")
    );
    assert_eq!(Problems::default().summary(), "No problems");
}

#[test]
fn chips_filter_and_toggle_back_to_all() {
    let mut problems = sample();
    assert_eq!(problems.chip(Filter::Errors), ("Errors 1".into(), false));
    problems.set_filter(Filter::Errors);
    assert!(problems.chip(Filter::Errors).1);
    let rows = problems.rows();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].template(), "ProblemError");
    problems.set_filter(Filter::Warnings);
    assert_eq!(problems.rows()[0].template(), "ProblemWarning");
    problems.set_filter(Filter::Warnings);
    assert_eq!(problems.filter, Filter::All);
    assert_eq!(problems.rows().len(), 2);
}

#[test]
fn full_output_shows_the_build_log() {
    let mut problems = sample();
    problems.log = vec!["> cargo build".into(), "error: could not compile".into()];
    problems.set_filter(Filter::Output);
    assert_eq!(problems.chip(Filter::Output), ("Full output".into(), true));
    assert_eq!(
        problems.rows(),
        vec![
            Row::Output {
                line: "> cargo build".into()
            },
            Row::Output {
                line: "error: could not compile".into()
            },
        ]
    );
    assert_eq!(problems.rows()[0].template(), "ProblemOutput");
}

#[test]
fn place_joins_the_root_and_counts_from_zero() {
    let place = sample().place(0).expect("the error has a place");
    assert_eq!(
        place.file,
        Path::new("C:/work/dbg_console").join("src/main.rs")
    );
    assert_eq!((place.line, place.byte), (7, 19));
    assert!(sample().place(9).is_none());
}

#[test]
fn a_diagnostic_without_location_has_no_place_and_an_empty_place_text() {
    let mut problems = Problems::default();
    let mut d = diagnostic(
        Severity::Error,
        "linking failed",
        1,
        "error: linking failed",
    );
    d.location = None;
    problems.populate(&[d], None);
    assert!(problems.place(0).is_none());
    match &problems.rows()[0] {
        Row::Problem { place, code, .. } => {
            assert!(place.is_empty());
            assert!(code.is_empty());
        }
        other => panic!("expected a problem row, got {other:?}"),
    }
}

const PANIC: &str = "thread 'main' panicked at src/form_main.rs:42:9:
attempt to divide by zero
stack backtrace:
   2: calc::form_main::FormMain::divide
             at ./src/form_main.rs:42:9
   3: std::rt::lang_start
             at /rustc/abc/library/std/src/rt.rs:1:1
   4: calc::form_main::FormMain::on_click
             at ./src/form_main.rs:30:13
";

fn panicked() -> Problems {
    let mut problems = sample();
    let root = std::env::temp_dir().join(format!("crawrad-problems-{}", std::process::id()));
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("src").join("form_main.rs"), "").unwrap();
    let sites = crawrad_project::panic_report::find_panics(PANIC, &root);
    problems.show_panics(sites, PANIC, &root);
    problems
}

#[test]
fn a_panic_leads_with_its_message_spot_and_the_projects_frames() {
    let p = panicked();
    assert_eq!(p.chip(Filter::Panics), ("Panics 1".to_string(), true));
    assert_eq!(p.reason, "The app panicked \u{2014} Run stopped");
    let rows = p.rows();
    let templates: Vec<&str> = rows.iter().map(Row::template).collect();
    assert_eq!(
        templates,
        [
            "ProblemPanic",
            "ProblemHeading",
            "ProblemFrame",
            "ProblemFrame",
            "ProblemHeading"
        ]
    );
    assert_eq!(
        rows[0],
        Row::Panic {
            site: 0,
            message: "attempt to divide by zero".into(),
            place: "src/form_main.rs:42:9".into(),
            meta: "thread 'main' \u{00b7} FormMain::divide".into(),
        }
    );
    assert!(
        matches!(&rows[3], Row::Frame { frame: 1, place, .. } if place == "src/form_main.rs:30:13")
    );
}

#[test]
fn all_shows_panics_then_diagnostics_and_a_new_build_forgets_them() {
    let mut p = panicked();
    p.set_filter(Filter::Panics);
    assert_eq!(p.rows().len(), 7);
    assert!(matches!(p.rows()[5], Row::Problem { .. }));
    p.populate(&[], None);
    assert!(p.panics.is_empty());
    assert_eq!(p.filter, Filter::All);
}

#[test]
fn a_click_on_a_panic_or_a_frame_goes_to_its_line() {
    let p = panicked();
    let at = p.panic_place(0, None).unwrap();
    assert!(at.file.ends_with(Path::new("src").join("form_main.rs")));
    assert_eq!((at.line, at.byte), (41, 8));
    assert_eq!(p.panic_place(0, Some(1)).unwrap().line, 29);
    assert!(p.panic_place(0, Some(9)).is_none());
    assert!(p.panic_place(3, None).is_none());
}

#[test]
fn a_panic_marks_its_own_file_only() {
    let p = panicked();
    let file = p.panic_place(0, None).unwrap().file;
    assert_eq!(
        p.panic_marks(&file),
        [(41, 8, "panicked: attempt to divide by zero".to_string())]
    );
    assert!(p.panic_marks(Path::new("src/other.rs")).is_empty());
}
