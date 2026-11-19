//! The session on its own thread, against the fake adapter: the IDE sends
//! commands and reads updates, and never waits on the debugger.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::controller::{Command, DebugController, Update};
use crate::presenter::AdapterText;
use crate::session::{DebugSession, LaunchConfig};
use crate::test_fake::{FakeAdapter, pair};

const T: Duration = Duration::from_secs(5);

fn config() -> LaunchConfig {
    LaunchConfig {
        program: PathBuf::from("C:/app/target/debug/app.exe"),
        ..LaunchConfig::default()
    }
}

fn breakpoints() -> BTreeMap<PathBuf, Vec<i64>> {
    BTreeMap::from([(PathBuf::from("C:/app/src/form_main.rs"), vec![12])])
}

/// The adapter's side of the handshake, as `lldb-dap` plays it.
fn handshake(adapter: &mut FakeAdapter) {
    let init = adapter.read_request();
    adapter.answer(&init, true, None, json!({}));
    let launch = adapter.read_request();
    adapter.event("initialized", Value::Null);
    let bps = adapter.read_request();
    adapter.answer(
        &bps,
        true,
        None,
        json!({ "breakpoints": [{ "verified": true, "line": 12 }] }),
    );
    let done = adapter.read_request();
    adapter.answer(&done, true, None, Value::Null);
    adapter.answer(&launch, true, None, Value::Null);
}

/// The stop's questions: the stack, then the scopes and locals of its top.
fn answer_stop(adapter: &mut FakeAdapter, line: i64) {
    let st = adapter.read_request();
    assert_eq!(st.command, "stackTrace");
    adapter.answer(&st, true, None, json!({ "stackFrames": [
        { "id": 9, "name": "FormMain::btn_ok_click", "line": line, "source": { "path": "C:/app/src/form_main.rs" } }
    ]}));
    let sc = adapter.read_request();
    assert_eq!(sc.command, "scopes");
    adapter.answer(
        &sc,
        true,
        None,
        json!({ "scopes": [{ "variablesReference": 44 }] }),
    );
    let vars = adapter.read_request();
    adapter.answer(
        &vars,
        true,
        None,
        json!({ "variables": [
            { "name": "n", "value": "42", "type": "int", "variablesReference": 0 }
        ]}),
    );
}

fn noop_waker() {}

/// Starts a controller over the fake adapter; the adapter comes back once the
/// handshake is done.
fn start() -> (DebugController, FakeAdapter) {
    let (conn, mut adapter) = pair();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        handshake(&mut adapter);
        let _ = tx.send(adapter);
    });
    let controller = DebugController::start(
        move || DebugSession::launch_over(conn, &config(), &breakpoints()),
        Box::new(AdapterText),
        noop_waker,
    );
    let adapter = rx.recv_timeout(T).expect("the handshake never finished");
    (controller, adapter)
}

/// Collects updates until one matches, failing after [`T`] — never a hang.
fn until(controller: &DebugController, seen: &mut Vec<Update>, done: impl Fn(&Update) -> bool) {
    let deadline = Instant::now() + T;
    loop {
        for update in controller.take_updates() {
            let hit = done(&update);
            seen.push(update);
            if hit {
                return;
            }
        }
        assert!(Instant::now() < deadline, "no such update; saw {seen:?}");
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// Runs the adapter's script on a thread of its own and hands the adapter back.
fn script(
    mut adapter: FakeAdapter,
    f: impl FnOnce(&mut FakeAdapter) + Send + 'static,
) -> std::sync::mpsc::Receiver<FakeAdapter> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        f(&mut adapter);
        let _ = tx.send(adapter);
    });
    rx
}

#[test]
fn a_stop_arrives_with_its_stack_and_locals_and_a_step_runs_to_the_next() {
    let (controller, adapter) = start();
    let mut seen = Vec::new();
    until(&controller, &mut seen, |u| {
        matches!(u, Update::Breakpoints(_))
    });
    assert_eq!(seen[0], Update::Started);

    let rx = script(adapter, |a| {
        a.event("output", json!({ "output": "hello\n" }));
        a.event("stopped", json!({ "reason": "breakpoint", "threadId": 1 }));
        answer_stop(a, 12);
    });
    until(&controller, &mut seen, |u| matches!(u, Update::Stopped(_)));
    let adapter = rx.recv_timeout(T).unwrap();
    assert!(seen.contains(&Update::Output("hello\n".into())));
    let Some(Update::Stopped(stop)) = seen.last() else {
        unreachable!()
    };
    assert_eq!(stop.reason, "breakpoint");
    assert_eq!(stop.frames[0].line, 12);
    assert_eq!(stop.locals[0].name, "n");
    assert_eq!(stop.locals[0].value, "42");

    // F8: the step leaves from line 12 (one stackTrace to note where) and
    // lands on 13.
    let rx = script(adapter, |a| {
        let st = a.read_request();
        assert_eq!(st.command, "stackTrace");
        a.answer(&st, true, None, json!({ "stackFrames": [
            { "id": 9, "name": "FormMain::btn_ok_click", "line": 12, "source": { "path": "C:/app/src/form_main.rs" } }
        ]}));
        let next = a.read_request();
        assert_eq!(next.command, "next");
        a.answer(&next, true, None, Value::Null);
        a.event("stopped", json!({ "reason": "step", "threadId": 1 }));
        answer_stop(a, 13);
    });
    assert!(controller.send(Command::StepOver));
    seen.clear();
    until(&controller, &mut seen, |u| matches!(u, Update::Stopped(_)));
    assert!(seen.contains(&Update::Running));
    let Some(Update::Stopped(stop)) = seen.last() else {
        unreachable!()
    };
    assert_eq!(stop.frames[0].line, 13);
    let mut adapter = rx.recv_timeout(T).unwrap();

    // The program ends by itself.
    adapter.event("exited", json!({ "exitCode": 0 }));
    adapter.event("terminated", Value::Null);
    seen.clear();
    // Continue first: the program was stopped.
    let rx = script(adapter, |a| {
        let st = a.read_request();
        assert_eq!(st.command, "stackTrace", "where the Continue leaves from");
        a.answer(&st, true, None, json!({ "stackFrames": [
            { "id": 9, "name": "FormMain::btn_ok_click", "line": 13, "source": { "path": "C:/app/src/form_main.rs" } }
        ]}));
        let c = a.read_request();
        assert_eq!(c.command, "continue");
        a.answer(&c, true, None, Value::Null);
    });
    assert!(controller.send(Command::Continue));
    until(&controller, &mut seen, |u| {
        matches!(u, Update::Terminated { .. })
    });
    assert_eq!(
        seen.last(),
        Some(&Update::Terminated { exit_code: Some(0) })
    );
    drop(rx);
}

#[test]
fn stop_while_the_program_runs_is_obeyed_within_a_slice() {
    let (controller, adapter) = start();
    let mut seen = Vec::new();
    until(&controller, &mut seen, |u| {
        matches!(u, Update::Breakpoints(_))
    });
    // The program runs and never stops by itself; Stop must still get through.
    let rx = script(adapter, |a| {
        let d = a.read_request();
        assert_eq!(d.command, "disconnect");
        assert_eq!(d.arguments["terminateDebuggee"], true);
        a.answer(&d, true, None, Value::Null);
    });
    let asked = Instant::now();
    assert!(controller.send(Command::Stop));
    until(&controller, &mut seen, |u| {
        matches!(u, Update::Terminated { .. })
    });
    assert!(
        asked.elapsed() < Duration::from_secs(2),
        "Stop took {:?}",
        asked.elapsed()
    );
    rx.recv_timeout(T)
        .expect("the adapter was never told to disconnect");
}

#[test]
fn a_debugger_that_does_not_start_says_so_and_ends() {
    let controller = DebugController::start(
        || Err(crate::connection::DapError::Closed),
        Box::new(AdapterText),
        noop_waker,
    );
    let mut seen = Vec::new();
    until(&controller, &mut seen, |u| matches!(u, Update::Failed(_)));
    let Some(Update::Failed(why)) = seen.last() else {
        unreachable!()
    };
    assert!(why.starts_with("the debugger did not start"), "{why}");
}

#[test]
fn dropping_the_controller_ends_the_program_it_drove() {
    let (controller, adapter) = start();
    let mut seen = Vec::new();
    until(&controller, &mut seen, |u| {
        matches!(u, Update::Breakpoints(_))
    });
    let rx = script(adapter, |a| {
        let d = a.read_request();
        assert_eq!(d.command, "disconnect");
        a.answer(&d, true, None, Value::Null);
    });
    drop(controller);
    rx.recv_timeout(T)
        .expect("closing the IDE must end the program under the debugger");
}

/// **A logpoint's message, as VS Code writes it**: `{expr}` replaced,
/// `\{` a literal brace, an unclosed `{` kept as written.
#[test]
fn a_log_message_fills_its_braces() {
    let value = |e: &str| match e {
        "total" => "42".to_string(),
        "name" => "\"Ana\"".to_string(),
        other => format!("<{other}>"),
    };
    assert_eq!(
        crate::render_log_message("total = {total}, name = { name }", value),
        "total = 42, name = \"Ana\""
    );
    assert_eq!(
        crate::render_log_message(r"\{literal} {total}", value),
        "{literal} 42"
    );
    assert_eq!(
        crate::render_log_message("open {total", value),
        "open {total"
    );
    assert_eq!(crate::render_log_message(r"a\b", value), r"a\b");
}

static WAKE_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn counting_waker() {
    WAKE_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
}

#[test]
fn waker_is_called_when_update_is_sent() {
    WAKE_COUNT.store(0, std::sync::atomic::Ordering::SeqCst);
    let (conn, mut adapter) = pair();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        handshake(&mut adapter);
        let _ = tx.send(adapter);
    });
    let controller = DebugController::start(
        move || DebugSession::launch_over(conn, &config(), &breakpoints()),
        Box::new(AdapterText),
        counting_waker,
    );
    let _adapter = rx.recv_timeout(T).expect("the handshake never finished");
    let mut seen = Vec::new();
    until(&controller, &mut seen, |u| {
        matches!(u, Update::Breakpoints(_))
    });
    assert!(WAKE_COUNT.load(std::sync::atomic::Ordering::SeqCst) >= 2);
    assert_eq!(
        seen.len(),
        WAKE_COUNT.load(std::sync::atomic::Ordering::SeqCst)
    );
}
