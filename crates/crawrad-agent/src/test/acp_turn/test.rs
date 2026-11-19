//! A turn folded from its events: text joined, tools tracked by name, and the
//! three ways it ends.

use super::{TurnEnd, collect_turn};
use crate::client::AcpStreamEvent;
use crate::markdown::ToolCallStatus;
use std::sync::mpsc::channel;

fn feed(events: Vec<AcpStreamEvent>) -> std::sync::mpsc::Receiver<AcpStreamEvent> {
    let (tx, rx) = channel();
    for e in events {
        tx.send(e).unwrap();
    }
    rx
}

#[test]
fn text_and_tools_fold_into_the_reply() {
    let rx = feed(vec![
        AcpStreamEvent::TextDelta("All ".into()),
        AcpStreamEvent::ToolCall {
            id: "t1".into(),
            name: "cargo test".into(),
            input: serde_json::json!({}),
        },
        AcpStreamEvent::ToolCallUpdate {
            id: "t1".into(),
            name: "cargo test".into(),
            status: ToolCallStatus::Success,
            summary: "ok".into(),
        },
        AcpStreamEvent::TextDelta("passed.".into()),
        AcpStreamEvent::Done {
            stop_reason: "end_turn".into(),
        },
        AcpStreamEvent::TextDelta("after the end".into()),
    ]);
    let mut seen = 0;
    let TurnEnd::Reply(text, tools) = collect_turn(&rx, || false, |_| seen += 1, |_| {}, |_| {})
    else {
        panic!("the turn should end with a reply");
    };
    assert_eq!(
        seen, 5,
        "every event up to the end is reported, before folding"
    );
    assert_eq!(text, "All passed.");
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].status, ToolCallStatus::Success);
    assert_eq!(tools[0].arguments, "ok");
}

#[test]
fn an_error_fails_and_a_cancel_cancels() {
    let rx = feed(vec![AcpStreamEvent::Error("gone".into())]);
    assert_eq!(
        collect_turn(&rx, || false, |_| {}, |_| {}, |_| {}),
        TurnEnd::Failed("gone".into())
    );
    let rx = feed(vec![AcpStreamEvent::TextDelta("x".into())]);
    assert_eq!(
        collect_turn(&rx, || true, |_| {}, |_| {}, |_| {}),
        TurnEnd::Cancelled
    );
}

#[test]
fn an_update_with_a_new_title_still_finishes_its_card() {
    // Claude names the call `Terminal`, then updates it under the command line.
    let rx = feed(vec![
        AcpStreamEvent::ToolCall {
            id: "c1".into(),
            name: "Terminal".into(),
            input: serde_json::json!({}),
        },
        AcpStreamEvent::ToolCallUpdate {
            id: "c1".into(),
            name: "`cargo build`".into(),
            status: ToolCallStatus::Success,
            summary: String::new(),
        },
        AcpStreamEvent::Done {
            stop_reason: "end_turn".into(),
        },
    ]);
    let TurnEnd::Reply(_, tools) = collect_turn(&rx, || false, |_| {}, |_| {}, |_| {}) else {
        panic!("the turn should end with a reply");
    };
    assert_eq!(tools.len(), 1, "one call, one card");
    assert_eq!(tools[0].name, "Terminal");
    assert_eq!(
        tools[0].status,
        ToolCallStatus::Success,
        "not stuck at Running"
    );
    assert_eq!(
        tools[0].arguments, "`cargo build`",
        "the command shows instead of {{}}"
    );
}

#[test]
fn text_after_a_tool_starts_a_paragraph_when_the_sentence_ended() {
    let rx = feed(vec![
        AcpStreamEvent::TextDelta("I will compile it first.".into()),
        AcpStreamEvent::ToolCall {
            id: "c1".into(),
            name: "Terminal".into(),
            input: serde_json::json!({}),
        },
        AcpStreamEvent::TextDelta("Now the answer.".into()),
        AcpStreamEvent::Done {
            stop_reason: "end_turn".into(),
        },
    ]);
    let TurnEnd::Reply(text, _) = collect_turn(&rx, || false, |_| {}, |_| {}, |_| {}) else {
        panic!("the turn should end with a reply");
    };
    assert_eq!(text, "I will compile it first.\n\nNow the answer.");
}

#[test]
fn a_write_file_event_reaches_on_write() {
    let req = std::sync::Arc::new(crate::file_write::FileWriteRequest::new(
        std::path::PathBuf::from("foo.rs"),
        "content".into(),
        |_| {},
    ));
    let rx = feed(vec![
        AcpStreamEvent::WriteFile(std::sync::Arc::clone(&req)),
        AcpStreamEvent::Done {
            stop_reason: "end_turn".into(),
        },
    ]);
    let mut written = None;
    let TurnEnd::Reply(..) = collect_turn(&rx, || false, |_| {}, |_| {}, |r| written = Some(r))
    else {
        panic!("the turn should end with a reply");
    };
    assert!(written.is_some());
    assert_eq!(written.unwrap().path, std::path::PathBuf::from("foo.rs"));
}
