//! The session against the fake adapter: the handshake order, the state
//! machine, the commands, and the stack/locals path.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};

use crate::connection::DapError;
use crate::presenter::{ValuePresenter, ValueSource};
use crate::protocol::Variable;
use crate::session::{DebugSession, LaunchConfig, SessionState};
use crate::test_fake::{FakeAdapter, pair};

const T: Duration = Duration::from_secs(5);

/// Runs the adapter's side of a test on its own thread, and hands back its
/// result through [`finish`] — never through a bare `join`: when the session
/// does not send what the script waits for, the test must FAIL within the
/// timeout, not hang the gate (a mutation proved the `join` version hangs).
fn script<R: Send + 'static>(
    f: impl FnOnce() -> R + Send + 'static,
) -> std::sync::mpsc::Receiver<R> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx
}

fn finish<R>(rx: std::sync::mpsc::Receiver<R>) -> R {
    rx.recv_timeout(T)
        .expect("the fake adapter's script never finished: the session did not send what it should")
}

fn config() -> LaunchConfig {
    LaunchConfig {
        program: PathBuf::from("C:/app/target/debug/app.exe"),
        cwd: Some(PathBuf::from("C:/app")),
        ..LaunchConfig::default()
    }
}

fn breakpoints() -> BTreeMap<PathBuf, Vec<i64>> {
    BTreeMap::from([(PathBuf::from("C:/app/src/form_main.rs"), vec![11])])
}

/// Plays the adapter's side of the handshake the way `lldb-dap` does: the
/// `launch` answer is held until `configurationDone`, and the program prints a
/// line BEFORE `initialized`. Returns the commands it saw, in order.
fn handshake(adapter: &mut FakeAdapter, seen: &Arc<Mutex<Vec<String>>>) {
    let next = |adapter: &mut FakeAdapter| {
        let r = adapter.read_request();
        seen.lock().unwrap().push(r.command.clone());
        r
    };
    let init = next(adapter);
    adapter.answer(
        &init,
        true,
        None,
        json!({ "supportsReadMemoryRequest": true }),
    );
    let launch = next(adapter);
    assert_eq!(launch.arguments["program"], "C:/app/target/debug/app.exe");
    assert_eq!(launch.arguments["cwd"], "C:/app");
    adapter.event(
        "output",
        json!({ "category": "stdout", "output": "first line\n" }),
    );
    adapter.event("initialized", Value::Null);
    let bps = next(adapter);
    assert_eq!(bps.arguments["source"]["path"], "C:/app/src/form_main.rs");
    // The adapter moves the breakpoint to the next line with code.
    adapter.answer(
        &bps,
        true,
        None,
        json!({ "breakpoints": [{ "verified": true, "line": 12 }] }),
    );
    let done = next(adapter);
    adapter.answer(&done, true, None, Value::Null);
    adapter.answer(&launch, true, None, Value::Null);
}

fn start() -> (DebugSession, FakeAdapter, Vec<String>) {
    let (conn, mut adapter) = pair();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let handshake_done = {
        let seen = Arc::clone(&seen);
        script(move || {
            handshake(&mut adapter, &seen);
            adapter
        })
    };
    let session =
        DebugSession::launch_over(conn, &config(), &breakpoints()).expect("the session starts");
    let adapter = finish(handshake_done);
    let seen = seen.lock().unwrap().clone();
    (session, adapter, seen)
}

#[test]
fn the_handshake_goes_in_the_protocols_order() {
    let (session, _adapter, seen) = start();
    assert_eq!(
        seen,
        [
            "initialize",
            "launch",
            "setBreakpoints",
            "configurationDone"
        ]
    );
    assert_eq!(session.state(), &SessionState::Running);
}

#[test]
fn an_output_before_initialized_is_kept_not_dropped() {
    let (session, _adapter, _) = start();
    assert_eq!(session.output(), ["first line\n"]);
}

#[test]
fn a_breakpoint_records_where_the_adapter_put_it() {
    let (session, _adapter, _) = start();
    let bps = &session.breakpoints()[&PathBuf::from("C:/app/src/form_main.rs")];
    assert_eq!(bps.len(), 1);
    assert_eq!(bps[0].requested_line, 11);
    assert_eq!(bps[0].line, Some(12));
    assert!(bps[0].verified);
}

#[test]
fn stop_step_continue_and_the_end() {
    let (mut session, mut adapter, _) = start();
    adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 7 }));
    assert_eq!(
        session.wait_for_stop(T).unwrap(),
        &SessionState::Stopped {
            thread_id: 7,
            reason: "breakpoint".into()
        }
    );

    let done = script(move || {
        answer_stack(&mut adapter, 12);
        let next = adapter.read_request();
        assert_eq!(next.command, "next");
        assert_eq!(next.arguments["threadId"], 7);
        assert_eq!(next.arguments["granularity"], "line");
        adapter.answer(&next, true, None, Value::Null);
        adapter.event("stopped", json!({ "reason": "step", "threadId": 7 }));
        answer_stack(&mut adapter, 13); // where the Continue leaves from
        let cont = adapter.read_request();
        assert_eq!(cont.command, "continue");
        adapter.answer(&cont, true, None, Value::Null);
        adapter.event("exited", json!({ "exitCode": 0 }));
        adapter.event("terminated", Value::Null);
        adapter
    });
    session.step_over().unwrap();
    assert_eq!(session.state(), &SessionState::Running);
    assert_eq!(
        session.wait_for_stop(T).unwrap(),
        &SessionState::Stopped {
            thread_id: 7,
            reason: "step".into()
        }
    );
    session.continue_().unwrap();
    assert_eq!(session.wait_for_stop(T).unwrap(), &SessionState::Terminated);
    assert_eq!(session.exit_code(), Some(0));
    finish(done);
}

#[test]
fn stepping_a_running_program_is_refused_without_asking_the_adapter() {
    let (mut session, _adapter, _) = start();
    assert!(matches!(session.step_into(), Err(DapError::Failed { .. })));
    assert_eq!(session.state(), &SessionState::Running);
}

/// Shows every `probe::Point` as `P`, to see the presenter being used.
struct PointPresenter;
impl ValuePresenter for PointPresenter {
    fn present(&self, v: &Variable, _s: &dyn ValueSource) -> Option<String> {
        (v.type_name.as_deref() == Some("probe::Point")).then(|| "P".to_string())
    }
}

#[test]
fn the_stack_and_the_locals_of_a_stop() {
    let (mut session, mut adapter, _) = start();
    adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 1 }));
    session.wait_for_stop(T).unwrap();
    let done = script(move || {
        let st = adapter.read_request();
        assert_eq!(st.command, "stackTrace");
        adapter.answer(&st, true, None, json!({ "stackFrames": [
            { "id": 9, "name": "FormMain::btn_ok_click", "line": 12, "source": { "path": "C:/app/src/form_main.rs" } },
            { "id": 10, "name": "App::handle_actions", "line": 681, "source": { "path": "C:/app/src/form_main.designer.rs" } }
        ]}));
        let sc = adapter.read_request();
        assert_eq!(sc.command, "scopes");
        assert_eq!(sc.arguments["frameId"], 9);
        adapter.answer(
            &sc,
            true,
            None,
            json!({ "scopes": [{ "name": "Locals", "variablesReference": 44 }] }),
        );
        let vars = adapter.read_request();
        assert_eq!(vars.arguments["variablesReference"], 44);
        adapter.answer(&vars, true, None, json!({ "variables": [
            { "name": "p", "value": "probe::Point", "type": "probe::Point", "variablesReference": 0 },
            { "name": "n", "value": "42", "type": "int", "variablesReference": 0 }
        ]}));
    });
    let frames = session.stack(1, 20).unwrap();
    assert_eq!(frames[0].name, "FormMain::btn_ok_click");
    assert_eq!(frames[0].line, 12);
    assert_eq!(
        frames[1].path.as_deref(),
        Some(std::path::Path::new("C:/app/src/form_main.designer.rs"))
    );
    let locals = session.locals(frames[0].id).unwrap();
    assert_eq!(
        session.present(&locals[0], frames[0].id, &PointPresenter),
        "P"
    );
    assert_eq!(
        session.present(&locals[1], frames[0].id, &PointPresenter),
        "42"
    );
    finish(done);
}

#[test]
fn an_adapter_that_dies_ends_the_session() {
    let (mut session, adapter, _) = start();
    drop(adapter);
    assert!(matches!(session.wait_for_stop(T), Err(DapError::Closed)));
    assert_eq!(session.state(), &SessionState::Terminated);
}

/// Answers the `stackTrace` a step asks for, with the top frame on `line`.
fn answer_stack(adapter: &mut FakeAdapter, line: i64) {
    let st = adapter.read_request();
    assert_eq!(st.command, "stackTrace");
    adapter.answer(&st, true, None, json!({ "stackFrames": [
        { "id": 1, "name": "app::main", "line": line, "source": { "path": "C:/app/src/main.rs" } }
    ]}));
}

#[test]
fn a_step_that_hits_the_same_line_s_breakpoint_again_goes_on_to_the_next_line() {
    // Measured with lldb-dap on 30/09/2026: a breakpoint on a `println!` line
    // is bound to several addresses, and `next` stopped on it again, twice.
    let (mut session, mut adapter, _) = start();
    adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 7 }));
    session.wait_for_stop(T).unwrap();
    let done = script(move || {
        answer_stack(&mut adapter, 13); // where the step leaves from
        let mut nexts = 0;
        for stop_reason in ["breakpoint", "breakpoint", "step"] {
            let next = adapter.read_request();
            assert_eq!(next.command, "next");
            nexts += 1;
            adapter.answer(&next, true, None, Value::Null);
            adapter.event("stopped", json!({ "reason": stop_reason, "threadId": 7 }));
            if stop_reason == "breakpoint" {
                answer_stack(&mut adapter, 13); // same line: not a real stop
            }
        }
        nexts
    });
    session.step_over().unwrap();
    assert_eq!(
        session.wait_for_stop(T).unwrap(),
        &SessionState::Stopped {
            thread_id: 7,
            reason: "step".into()
        }
    );
    assert_eq!(
        finish(done),
        3,
        "the step is issued again until the line changes"
    );
}

#[test]
fn a_step_that_hits_a_breakpoint_elsewhere_stops_there() {
    // F8 over a call whose body has a breakpoint MUST stop in that body.
    let (mut session, mut adapter, _) = start();
    adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 7 }));
    session.wait_for_stop(T).unwrap();
    let done = script(move || {
        answer_stack(&mut adapter, 13);
        let next = adapter.read_request();
        adapter.answer(&next, true, None, Value::Null);
        adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 7 }));
        answer_stack(&mut adapter, 40); // another line: a real breakpoint
        adapter
    });
    session.step_over().unwrap();
    assert_eq!(
        session.wait_for_stop(T).unwrap(),
        &SessionState::Stopped {
            thread_id: 7,
            reason: "breakpoint".into()
        }
    );
    finish(done);
}

/// Answers a `stackTrace` with the top frame on `line`, at `instruction`.
fn answer_stack_at(adapter: &mut FakeAdapter, line: i64, instruction: &str) {
    let st = adapter.read_request();
    assert_eq!(st.command, "stackTrace");
    adapter.answer(
        &st,
        true,
        None,
        json!({ "stackFrames": [
            { "id": 1, "name": "app::FormMain::btn_ok_click", "line": line,
              "source": { "path": "C:/app/src/form_main.rs" },
              "instructionPointerReference": instruction }
        ]}),
    );
}

#[test]
fn a_continue_goes_past_the_other_addresses_of_the_line_it_left() {
    // Measured on 30/09/2026: F9 from a breakpoint on `format!(…)` in a click
    // handler stopped on the same line again — its other address.
    let (mut session, mut adapter, _) = start();
    adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 7 }));
    session.wait_for_stop(T).unwrap();
    let done = script(move || {
        answer_stack_at(&mut adapter, 12, "0x1000"); // where F9 leaves from
        let mut continues = 0;
        for (reason, at) in [("breakpoint", Some("0x1040")), ("breakpoint", None)] {
            let c = adapter.read_request();
            assert_eq!(c.command, "continue");
            continues += 1;
            adapter.answer(&c, true, None, Value::Null);
            adapter.event("stopped", json!({ "reason": reason, "threadId": 7 }));
            match at {
                Some(at) => answer_stack_at(&mut adapter, 12, at), // same line, elsewhere
                None => answer_stack_at(&mut adapter, 40, "0x2000"), // a real one
            }
        }
        continues
    });
    session.continue_().unwrap();
    session.wait_for_stop(T).unwrap();
    assert_eq!(
        finish(done),
        2,
        "the line's other address is continued past"
    );
}

#[test]
fn a_continue_that_comes_round_to_the_same_instruction_stops_there() {
    // A breakpoint inside a loop: the next turn is a real stop, even on the
    // very line F9 left from.
    let (mut session, mut adapter, _) = start();
    adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 7 }));
    session.wait_for_stop(T).unwrap();
    let done = script(move || {
        answer_stack_at(&mut adapter, 12, "0x1000");
        let c = adapter.read_request();
        assert_eq!(c.command, "continue");
        adapter.answer(&c, true, None, Value::Null);
        adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 7 }));
        answer_stack_at(&mut adapter, 12, "0x1000");
        adapter
    });
    session.continue_().unwrap();
    assert_eq!(
        session.wait_for_stop(T).unwrap(),
        &SessionState::Stopped {
            thread_id: 7,
            reason: "breakpoint".into()
        }
    );
    finish(done);
}
