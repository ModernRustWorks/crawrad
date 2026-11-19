//! The connection against a fake adapter in-process, over `std::io::pipe`.
//!
//! No real adapter, no process: each test scripts what the adapter says and
//! checks where it lands.

use std::time::Duration;

use serde_json::{Value, json};

use crate::connection::DapError;
use crate::presenter::ValueSource;
use crate::protocol::{ProtocolMessage, Request};
use crate::test_fake::pair;

const T: Duration = Duration::from_secs(5);

#[test]
fn a_response_goes_to_the_request_it_names_even_out_of_order() {
    let (conn, mut adapter) = pair();
    let first = conn.send_request("threads", json!({})).unwrap();
    let second = conn
        .send_request("stackTrace", json!({ "threadId": 1 }))
        .unwrap();
    let a = adapter.read_request();
    let b = adapter.read_request();
    // The adapter answers the SECOND request first — `launch` really does.
    adapter.answer(&b, true, None, json!({ "who": "stackTrace" }));
    adapter.answer(&a, true, None, json!({ "who": "threads" }));
    assert_eq!(first.wait(T).unwrap()["who"], "threads");
    assert_eq!(second.wait(T).unwrap()["who"], "stackTrace");
}

#[test]
fn a_failed_response_is_an_error_carrying_the_adapters_message() {
    let (conn, mut adapter) = pair();
    let pending = conn
        .send_request("launch", json!({ "program": "nope.exe" }))
        .unwrap();
    let r = adapter.read_request();
    adapter.answer(&r, false, Some("program not found"), Value::Null);
    match pending.wait(T) {
        Err(DapError::Failed { command, message }) => {
            assert_eq!(command, "launch");
            assert_eq!(message, "program not found");
        }
        other => panic!("expected Failed, got {other:?}"),
    }
}

#[test]
fn events_arrive_in_order_and_wait_event_skips_to_the_one_asked() {
    let (conn, mut adapter) = pair();
    adapter.event("output", json!({ "output": "hello\n" }));
    adapter.event("initialized", Value::Null);
    adapter.event("stopped", json!({ "reason": "breakpoint", "threadId": 7 }));
    assert_eq!(conn.next_event(T).unwrap().event, "output");
    let stopped = conn.wait_event("stopped", T).unwrap();
    assert_eq!(stopped.body["reason"], "breakpoint");
    assert_eq!(stopped.body["threadId"], 7);
}

#[test]
fn a_reverse_request_is_answered_so_the_adapter_never_waits_forever() {
    let (_conn, mut adapter) = pair();
    adapter.send(ProtocolMessage::Request(Request {
        seq: 42,
        command: "runInTerminal".into(),
        arguments: json!({ "args": ["x"] }),
    }));
    // Read on another thread: if the client never answers, this test must
    // FAIL within the timeout, not hang the whole gate.
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(adapter.read_response());
    });
    let answer = rx
        .recv_timeout(T)
        .expect("the client never answered the reverse request");
    assert_eq!(answer.request_seq, 42);
    assert!(!answer.success);
    assert_eq!(answer.command, "runInTerminal");
}

#[test]
fn when_the_adapter_goes_away_a_waiting_request_gets_closed_not_a_hang() {
    let (conn, mut adapter) = pair();
    let pending = conn
        .send_request("continue", json!({ "threadId": 1 }))
        .unwrap();
    let _ = adapter.read_request();
    drop(adapter); // the adapter process died
    let started = std::time::Instant::now();
    assert!(matches!(
        pending.wait(Duration::from_secs(30)),
        Err(DapError::Closed)
    ));
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "it waited for the timeout instead"
    );
    std::thread::sleep(Duration::from_millis(50));
    assert!(!conn.is_alive());
    assert!(matches!(
        conn.send_request("threads", json!({})),
        Err(DapError::Closed)
    ));
}

#[test]
fn a_message_that_cannot_be_read_is_skipped_and_the_next_one_still_lands() {
    let (conn, mut adapter) = pair();
    let pending = conn.send_request("threads", json!({})).unwrap();
    let r = adapter.read_request();
    adapter.send_raw("{ this is not a DAP message");
    adapter.answer(&r, true, None, json!({ "threads": [] }));
    assert!(pending.wait(T).unwrap()["threads"].is_array());
}

#[test]
fn the_connection_answers_a_presenters_questions() {
    let (conn, mut adapter) = pair();
    let script = std::thread::spawn(move || {
        let r = adapter.read_request();
        assert_eq!(r.command, "variables");
        adapter.answer(&r, true, None, json!({ "variables": [
            { "name": "len", "value": "13", "type": "unsigned long long", "variablesReference": 0,
              "memoryReference": "0x8DFD1FF350" }
        ]}));
        let r = adapter.read_request();
        assert_eq!(r.command, "readMemory");
        assert_eq!(r.arguments["memoryReference"], "0x0000028d4f46d9c0");
        adapter.answer(
            &r,
            true,
            None,
            json!({ "address": "0x0000028d4f46d9c0", "data": "T2zDoQ==" }),
        );
        let r = adapter.read_request();
        assert_eq!(r.command, "evaluate");
        assert_eq!(r.arguments["frameId"], 3);
        adapter.answer(
            &r,
            true,
            None,
            json!({ "result": "20", "type": "int", "variablesReference": 0 }),
        );
    });
    let kids = conn.children(9).unwrap();
    assert_eq!(kids[0].name, "len");
    assert_eq!(kids[0].value, "13");
    assert_eq!(kids[0].memory_reference.as_deref(), Some("0x8DFD1FF350"));
    let bytes = conn.read_memory("0x0000028d4f46d9c0", 4).unwrap();
    assert_eq!(String::from_utf8(bytes).unwrap(), "Olá");
    let v = conn.evaluate("((int*)p)[1]", Some(3)).unwrap();
    assert_eq!(v.value, "20");
    script.join().unwrap();
}
