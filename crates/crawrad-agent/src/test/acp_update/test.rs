//! Unit tests and real turn replay for ACP update parsing.
#![forbid(unsafe_code)]

use std::collections::HashMap;

use super::parse_update;
use crate::client::AcpStreamEvent;
use crate::markdown::ToolCallStatus;

#[test]
fn test_agent_message_chunk_text_and_non_text() {
    let mut titles = HashMap::new();

    let valid_chunk = serde_json::json!({
        "sessionUpdate": "agent_message_chunk",
        "content": {
            "type": "text",
            "text": "Hello, world!"
        }
    });
    assert_eq!(
        parse_update(&valid_chunk, &mut titles),
        Some(AcpStreamEvent::TextDelta("Hello, world!".to_string()))
    );

    let non_text_chunk = serde_json::json!({
        "sessionUpdate": "agent_message_chunk",
        "content": {
            "type": "image",
            "data": "base64data"
        }
    });
    assert_eq!(parse_update(&non_text_chunk, &mut titles), None);
}

#[test]
fn test_agent_thought_chunk() {
    let mut titles = HashMap::new();

    let thought_chunk = serde_json::json!({
        "sessionUpdate": "agent_thought_chunk",
        "content": {
            "type": "text",
            "text": "Reflecting on code structure..."
        }
    });
    assert_eq!(
        parse_update(&thought_chunk, &mut titles),
        Some(AcpStreamEvent::ThoughtDelta(
            "Reflecting on code structure...".to_string()
        ))
    );
}

#[test]
fn test_tool_call_remembers_title_and_status() {
    let mut titles = HashMap::new();

    let call = serde_json::json!({
        "sessionUpdate": "tool_call",
        "toolCallId": "call-42",
        "title": "Terminal Exec",
        "kind": "execute",
        "status": "pending",
        "rawInput": { "command": "cargo build" }
    });

    let ev = parse_update(&call, &mut titles);
    assert_eq!(
        ev,
        Some(AcpStreamEvent::ToolCall {
            id: "call-42".to_string(),
            name: "Terminal Exec".to_string(),
            input: serde_json::json!({ "command": "cargo build" }),
        })
    );

    assert_eq!(titles.get("call-42"), Some(&"Terminal Exec".to_string()));
    assert_eq!(titles.get("status:call-42"), Some(&"pending".to_string()));
}

#[test]
fn test_tool_call_update_title_and_status_tracking() {
    let mut titles = HashMap::new();

    // 1. Initial call
    let call = serde_json::json!({
        "sessionUpdate": "tool_call",
        "toolCallId": "call-1",
        "title": "Initial Title",
        "status": "pending",
        "rawInput": {}
    });
    parse_update(&call, &mut titles);

    // 2. Title change without status keeps old status (Pending)
    let update_title = serde_json::json!({
        "sessionUpdate": "tool_call_update",
        "toolCallId": "call-1",
        "title": "Updated Title"
    });
    assert_eq!(
        parse_update(&update_title, &mut titles),
        Some(AcpStreamEvent::ToolCallUpdate {
            id: "call-1".to_string(),
            name: "Updated Title".to_string(),
            status: ToolCallStatus::Pending,
            summary: String::new(),
        })
    );

    // 3. Status change to in_progress keeps updated title
    let update_status_running = serde_json::json!({
        "sessionUpdate": "tool_call_update",
        "toolCallId": "call-1",
        "status": "in_progress"
    });
    assert_eq!(
        parse_update(&update_status_running, &mut titles),
        Some(AcpStreamEvent::ToolCallUpdate {
            id: "call-1".to_string(),
            name: "Updated Title".to_string(),
            status: ToolCallStatus::Running,
            summary: String::new(),
        })
    );

    // 4. Status change to completed (Success)
    let update_status_completed = serde_json::json!({
        "sessionUpdate": "tool_call_update",
        "toolCallId": "call-1",
        "status": "completed"
    });
    assert_eq!(
        parse_update(&update_status_completed, &mut titles),
        Some(AcpStreamEvent::ToolCallUpdate {
            id: "call-1".to_string(),
            name: "Updated Title".to_string(),
            status: ToolCallStatus::Success,
            summary: String::new(),
        })
    );

    // 5. Update with neither title nor status yields None
    let update_noop = serde_json::json!({
        "sessionUpdate": "tool_call_update",
        "toolCallId": "call-1"
    });
    assert_eq!(parse_update(&update_noop, &mut titles), None);

    // 6. Status failed
    let update_failed = serde_json::json!({
        "sessionUpdate": "tool_call_update",
        "toolCallId": "call-1",
        "status": "failed"
    });
    assert_eq!(
        parse_update(&update_failed, &mut titles),
        Some(AcpStreamEvent::ToolCallUpdate {
            id: "call-1".to_string(),
            name: "Updated Title".to_string(),
            status: ToolCallStatus::Failed,
            summary: String::new(),
        })
    );
}

#[test]
fn test_ignored_updates_return_none() {
    let mut titles = HashMap::new();

    let ignored_updates = [
        serde_json::json!({ "sessionUpdate": "user_message_chunk" }),
        serde_json::json!({ "sessionUpdate": "plan" }),
        serde_json::json!({ "sessionUpdate": "available_commands_update" }),
        serde_json::json!({ "sessionUpdate": "current_mode_update" }),
        serde_json::json!({ "sessionUpdate": "config_option_update" }),
        serde_json::json!({ "sessionUpdate": "session_info_update" }),
        serde_json::json!({ "sessionUpdate": "usage_update" }),
        serde_json::json!({ "sessionUpdate": "something_unrecognized" }),
    ];

    for update in &ignored_updates {
        assert_eq!(parse_update(update, &mut titles), None);
    }
}

#[test]
fn test_real_claude_turn_replay() {
    let lines = include_str!("real_claude_turn.jsonl");
    let mut titles = HashMap::new();
    let mut events = Vec::new();

    for line in lines.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let parsed: serde_json::Value =
            serde_json::from_str(trimmed).expect("valid json in jsonl line");
        let update = parsed
            .get("params")
            .and_then(|p| p.get("update"))
            .expect("params.update present");
        if let Some(ev) = parse_update(update, &mut titles) {
            events.push(ev);
        }
    }

    assert_eq!(
        events.len(),
        4,
        "Exactly 4 events expected: 1 tool_call, 2 tool_call_updates, 1 agent_message_chunk"
    );

    // 1. ToolCall { name: "Terminal" }
    match &events[0] {
        AcpStreamEvent::ToolCall { id, name, input } => {
            assert_eq!(id, "toolu_014igEwwzXpubVPJt94K9VA1");
            assert_eq!(name, "Terminal");
            assert_eq!(input, &serde_json::json!({}));
        }
        other => panic!("Event 0 must be ToolCall, got {other:?}"),
    }

    // 2. ToolCallUpdate named "echo hello" with status Pending
    match &events[1] {
        AcpStreamEvent::ToolCallUpdate {
            id,
            name,
            status,
            summary,
        } => {
            assert_eq!(id, "toolu_014igEwwzXpubVPJt94K9VA1");
            assert_eq!(name, "echo hello");
            assert_eq!(*status, ToolCallStatus::Pending);
            assert!(summary.is_empty());
        }
        other => panic!("Event 1 must be ToolCallUpdate Pending, got {other:?}"),
    }

    // 3. ToolCallUpdate with status Success (completed) keeping "echo hello" title
    match &events[2] {
        AcpStreamEvent::ToolCallUpdate {
            id,
            name,
            status,
            summary,
        } => {
            assert_eq!(id, "toolu_014igEwwzXpubVPJt94K9VA1");
            assert_eq!(name, "echo hello");
            assert_eq!(*status, ToolCallStatus::Success);
            assert!(summary.is_empty());
        }
        other => panic!("Event 2 must be ToolCallUpdate Success, got {other:?}"),
    }

    // 4. TextDelta("done.")
    match &events[3] {
        AcpStreamEvent::TextDelta(text) => {
            assert_eq!(text, "done.");
        }
        other => panic!("Event 3 must be TextDelta, got {other:?}"),
    }
}

#[test]
fn a_login_is_asked_only_on_the_protocol_s_auth_error() {
    use crate::client::{auth_method_ids, is_auth_required};
    let init = serde_json::json!({ "authMethods": [{ "id": "oauth-personal", "name": "Google" }, { "id": "api-key" }] });
    assert_eq!(auth_method_ids(&init), ["oauth-personal", "api-key"]);
    assert!(auth_method_ids(&serde_json::json!({})).is_empty());
    assert!(is_auth_required(
        r#"{"code":-32000,"message":"Authentication required"}"#
    ));
    assert!(!is_auth_required(
        r#"{"code":-32602,"message":"Invalid params"}"#
    ));
    assert!(!is_auth_required("ACP request `session/new` timed out"));
}
