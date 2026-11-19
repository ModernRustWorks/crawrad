//! The live status line: the five approved steps, the thought's one-line tail,
//! the tool count wording, a failed tool, text that is not a status, the CLI
//! wording, and the shell's tick.

use super::{StatusFeed, StatusLine, StatusTracker, StatusWatch, with_elapsed};
use crate::client::AcpStreamEvent;
use crate::markdown::ToolCallStatus;
use std::sync::mpsc::channel;
use std::time::Duration;

fn secs(n: u64) -> Duration {
    Duration::from_secs(n)
}

fn tool_call(id: &str, title: &str) -> AcpStreamEvent {
    AcpStreamEvent::ToolCall {
        id: id.into(),
        name: title.into(),
        input: serde_json::json!({}),
    }
}

fn tool_update(id: &str, title: &str, status: ToolCallStatus) -> AcpStreamEvent {
    AcpStreamEvent::ToolCallUpdate {
        id: id.into(),
        name: title.into(),
        status,
        summary: String::new(),
    }
}

fn line(line: &str, sub: &str, done: bool) -> StatusLine {
    StatusLine {
        line: line.into(),
        sub: sub.into(),
        done,
    }
}

#[test]
fn the_five_steps_read_as_approved() {
    let mut t = StatusTracker::new("Claude", false);
    assert_eq!(
        t.at(secs(0)),
        line(
            "ACP session started. Sending the prompt to Claude\u{2026}",
            "waiting for the first update",
            false
        )
    );

    assert!(t.on_event(&AcpStreamEvent::ThoughtDelta(
        "Looking for the tests".into()
    )));
    assert_eq!(
        t.at(secs(2)),
        line(
            "Thinking: Looking for the tests\u{2026}",
            "thought \u{b7} 2 s",
            false
        )
    );

    assert!(t.on_event(&tool_call("t1", "cargo test -p app")));
    assert_eq!(
        t.at(secs(3)),
        line(
            "Running tool: cargo test -p app",
            "tool_call \u{b7} in_progress \u{b7} 3 s",
            false
        )
    );
    assert!(!t.on_event(&tool_update(
        "t1",
        "cargo test -p app",
        ToolCallStatus::Running
    )));

    assert!(t.on_event(&tool_update(
        "t1",
        "cargo test -p app",
        ToolCallStatus::Success
    )));
    assert_eq!(
        t.at(secs(4)),
        line(
            "Tool done: cargo test -p app (ok)",
            "tool_call \u{b7} completed",
            false
        )
    );

    t.finish();
    assert_eq!(t.at(secs(5)), line("Done in 5 s \u{b7} 1 tool", "", true));
}

#[test]
fn the_thought_is_one_line_and_its_last_seventy_chars() {
    let mut t = StatusTracker::new("Claude", false);
    t.on_event(&AcpStreamEvent::ThoughtDelta(
        "First line\n\n  second".into(),
    ));
    assert_eq!(t.at(secs(1)).line, "Thinking: First line second\u{2026}");

    let long = "x".repeat(100) + "THE END";
    t.on_event(&AcpStreamEvent::ThoughtDelta(long));
    let shown = t.at(secs(1)).line;
    let tail = shown
        .strip_prefix("Thinking: ")
        .and_then(|s| s.strip_suffix('\u{2026}'))
        .expect("the thought keeps its frame");
    assert_eq!(tail.chars().count(), 70);
    assert!(tail.ends_with("THE END"));
    assert!(!tail.contains('\n'));
}

#[test]
fn the_tool_count_says_nothing_one_tool_or_n_tools() {
    let mut none = StatusTracker::new("Claude", false);
    none.finish();
    assert_eq!(none.at(secs(7)).line, "Done in 7 s");

    let mut two = StatusTracker::new("Claude", false);
    two.on_event(&tool_call("a", "read"));
    two.on_event(&tool_call("b", "read"));
    two.finish();
    assert_eq!(two.at(secs(9)).line, "Done in 9 s \u{b7} 2 tools");
}

#[test]
fn a_failed_tool_says_so() {
    let mut t = StatusTracker::new("Claude", false);
    t.on_event(&tool_call("t1", "cargo build"));
    t.on_event(&tool_update("t1", "cargo build", ToolCallStatus::Failed));
    assert_eq!(
        t.at(secs(1)),
        line(
            "Tool done: cargo build (failed)",
            "tool_call \u{b7} failed",
            false
        )
    );
}

#[test]
fn an_update_without_a_known_id_finds_the_tool_by_title() {
    let mut t = StatusTracker::new("Claude", false);
    t.on_event(&tool_call("t1", "cargo build"));
    t.on_event(&tool_update("", "cargo build", ToolCallStatus::Success));
    t.finish();
    assert_eq!(t.at(secs(1)).line, "Done in 1 s \u{b7} 1 tool");
}

#[test]
fn answer_text_does_not_change_the_line() {
    let mut t = StatusTracker::new("Claude", false);
    let before = t.at(secs(1));
    assert!(!t.on_event(&AcpStreamEvent::TextDelta("The answer".into())));
    assert_eq!(t.at(secs(1)), before);
}

#[test]
fn the_cli_wording() {
    let mut t = StatusTracker::new("Gemini", true);
    assert_eq!(
        t.at(secs(0)).line,
        "Sending the prompt to Gemini through the CLI\u{2026}"
    );
    t.finish();
    assert_eq!(t.at(secs(3)), line("Done in 3 s", "", true));
}

#[test]
fn the_shell_ticks_only_running_seconds() {
    let thought = line("Thinking: x\u{2026}", "thought \u{b7} 1 s", false);
    assert_eq!(with_elapsed(&thought, secs(4)).sub, "thought \u{b7} 4 s");
    let running = line(
        "Running tool: t",
        "tool_call \u{b7} in_progress \u{b7} 2 s",
        false,
    );
    assert_eq!(
        with_elapsed(&running, secs(12)).sub,
        "tool_call \u{b7} in_progress \u{b7} 12 s"
    );
    let waiting = line("ACP\u{2026}", "waiting for the first update", false);
    assert_eq!(with_elapsed(&waiting, secs(9)), waiting);
    let done = line("Done in 3 s", "", true);
    assert_eq!(with_elapsed(&done, secs(9)), done);
}

#[test]
fn the_feed_sends_what_the_watch_shows() {
    let (tx, mut watch) = StatusWatch::channel("Claude");
    assert!(watch.current().line.starts_with("ACP session started."));
    let mut feed = StatusFeed::start("Claude", false, tx);
    feed.on_event(&tool_call("t1", "cargo test"));
    assert_eq!(watch.poll().line, "Running tool: cargo test");
    feed.fall_back_to_cli();
    assert!(watch.poll().line.ends_with("through the CLI\u{2026}"));
    let last = feed.finish();
    assert!(last.done);
    assert_eq!(watch.poll(), last);

    let (tx, _rx) = channel();
    let mut quiet = StatusFeed::start("Claude", false, tx);
    assert!(quiet.finish().line.starts_with("Done in "));
}
