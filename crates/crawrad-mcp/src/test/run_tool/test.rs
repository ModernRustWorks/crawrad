//! The `build.*` and `run.*` answers, and the one program `run.start` keeps.

use super::{RunSlot, answer, in_project};
use serde_json::{Value, json};

fn status(a: &super::Answer) -> Value {
    a.0.data
        .as_ref()
        .map_or(Value::Null, |d| d["status"].clone())
}

/// A program that outlives the test's checks, on every runner.
fn long_lived() -> std::process::Command {
    if cfg!(windows) {
        let mut c = std::process::Command::new("cmd");
        c.args(["/C", "ping -n 30 127.0.0.1 > NUL"]);
        c
    } else {
        let mut c = std::process::Command::new("sleep");
        c.arg("30");
        c
    }
}

#[test]
fn no_project_refuses_before_cargo_runs() {
    let a = in_project(None, |_| unreachable!("cargo ran without a project"));
    assert!(a.0.is_error);
    assert_eq!(
        (a.0.data.unwrap()["code"].clone(), a.1),
        (json!("no-project"), "error")
    );
    let a = RunSlot::default().start(None, &json!({}));
    assert_eq!((a.0.is_error, a.1), (true, "error"));
}

#[test]
fn a_failed_build_answers_but_audits_as_an_error() {
    let ok = answer(Ok((json!({ "ok": true }), "built".into(), true)));
    assert_eq!((ok.0.is_error, ok.1), (false, "applied"));
    let failed = answer(Ok((json!({ "ok": false }), "errors".into(), false)));
    assert_eq!((failed.0.is_error, failed.1), (false, "error"));
    let spawn = answer(Err("no cargo".into()));
    assert_eq!((spawn.0.is_error, spawn.1), (true, "error"));
}

#[test]
fn run_start_keeps_one_program_until_run_stop() {
    let mut slot = RunSlot::default();
    assert_eq!(status(&slot.stop()), json!("not_running"));
    let started = slot.spawn(long_lived());
    assert_eq!((status(&started), started.1), (json!("started"), "applied"));
    let root = std::env::temp_dir();
    let again = slot.start(Some(root), &json!({}));
    assert_eq!(
        (status(&again), again.1),
        (json!("already_running"), "rejected")
    );
    let stopped = slot.stop();
    assert_eq!((status(&stopped), stopped.1), (json!("stopped"), "applied"));
    assert_eq!(slot.running(), None);
}
