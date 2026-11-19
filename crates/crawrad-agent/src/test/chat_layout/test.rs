use super::*;
use crate::session::ChatMessage;

#[test]
fn short_text_gets_large_indent() {
    let width = 300.0;
    let indent = user_bubble_indent("Hi", "19:19 ✓✓", width);
    assert!(
        indent > 140.0,
        "short text must have large left indent to stay compact on the right"
    );
}

#[test]
fn long_text_gets_minimum_indent() {
    let width = 300.0;
    let long_text = "This is a very long prompt written by the user to request a full refactoring of the visual component.";
    let indent = user_bubble_indent(long_text, "19:19 ✓✓", width);
    assert_eq!(
        indent, USER_BUBBLE_MIN_INDENT,
        "long text must reach the minimum indent"
    );
}

#[test]
fn multi_line_text_measured_by_longest_line() {
    let width = 300.0;
    let single_short = "Short";
    let multi_line =
        "Short\nThis is a significantly longer second line that determines width\nTiny";
    let indent_single = user_bubble_indent(single_short, "19:19 ✓✓", width);
    let indent_multi = user_bubble_indent(multi_line, "19:19 ✓✓", width);
    assert!(
        indent_multi < indent_single,
        "multi-line text indent must be sized according to its longest line"
    );
}

#[test]
fn delivery_mark_returns_codicon_glyphs() {
    assert_eq!(delivery_mark(false), "\u{eab2}");
    assert_eq!(delivery_mark(true), "\u{ebb1}");
}

#[test]
fn delivery_word_returns_single_or_double_check() {
    assert_eq!(delivery_word(false), "✓");
    assert_eq!(delivery_word(true), "✓✓");
}

#[test]
fn group_chat_turns_pairs_user_and_reply_with_meta() {
    let mut u = ChatMessage::user("u1", "Hello");
    u.sent_at = Some("10:00".into());
    let mut a = ChatMessage::assistant("a1", "Hi there!");
    a.sent_at = Some("10:01".into());

    let turns = group_chat_turns(&[u, a], "Copilot");
    assert_eq!(turns.len(), 1);
    let t = &turns[0];
    assert_eq!(t.user_text.as_deref(), Some("Hello"));
    assert_eq!(t.user_meta.as_deref(), Some("10:00"));
    assert_eq!(t.user_mark, Some("\u{ebb1}"));
    assert_eq!(t.reply_text.as_deref(), Some("Hi there!"));
    assert_eq!(t.reply_meta.as_deref(), Some("10:01"));
}

#[test]
fn turns_json_produces_expected_schema() {
    let mut u = ChatMessage::user("u1", "What is Rust?");
    u.sent_at = None; // Legacy message without timestamp
    let mut a = ChatMessage::assistant("a1", "Rust is a systems language.");
    a.sent_at = Some("12:30".into());

    let val = turns_json(
        &[u, a],
        "Copilot",
        &crate::acp_live::LiveStatus::Ready {
            session: "mock-sess".into(),
        },
        None,
    );
    let turns_array = val
        .get("turns")
        .and_then(|v| v.as_array())
        .expect("turns array");
    assert_eq!(turns_array.len(), 1);

    let first = &turns_array[0];
    assert_eq!(first["user"]["text"], "What is Rust?");
    assert_eq!(first["user"]["meta"], "✓✓");
    assert_eq!(first["reply"]["text"], "Rust is a systems language.");
    assert_eq!(first["reply"]["meta"], "12:30");
    assert_eq!(val["agent"]["live"], "ready");
    assert_eq!(val["agent"]["session"], "mock-sess");
    assert_eq!(val["status"], serde_json::Value::Null);
    assert_eq!(first["reply"]["status"], serde_json::Value::Null);
}

#[test]
fn turns_json_carries_the_live_and_the_final_status_lines() {
    use crate::acp_status::StatusLine;
    let done = StatusLine {
        line: "Done in 4 s · 1 tool".into(),
        sub: String::new(),
        done: true,
    };
    let u = ChatMessage::user("u1", "Run the tests");
    let a = ChatMessage::assistant("a1", "All green").with_status_line(Some(done.clone()));
    let live = StatusLine {
        line: "Running tool: cargo test".into(),
        sub: "tool_call · in_progress · 2 s".into(),
        done: false,
    };
    let idle = crate::acp_live::LiveStatus::Idle;
    let val = turns_json(&[u, a], "Claude", &idle, Some(&live));
    assert_eq!(val["status"]["line"], "Running tool: cargo test");
    assert_eq!(val["status"]["sub"], "tool_call · in_progress · 2 s");
    assert_eq!(val["status"]["done"], false);
    let reply = &val["turns"][0]["reply"]["status"];
    assert_eq!(reply["line"], "Done in 4 s · 1 tool");
    assert_eq!(reply["done"], true);
    let turns = group_chat_turns(&val_messages_roundtrip(&done), "Claude");
    assert_eq!(turns[0].status_line.as_ref(), Some(&done));
}

/// A reply saved with its status line reads it back (old sessions without one load too).
fn val_messages_roundtrip(done: &crate::acp_status::StatusLine) -> Vec<ChatMessage> {
    let a = ChatMessage::assistant("a1", "ok").with_status_line(Some(done.clone()));
    let json = serde_json::to_string(&[a]).expect("serialises");
    let old = r#"{"id":"a0","sender":"assistant","text":"old","status":"complete"}"#;
    let legacy: ChatMessage = serde_json::from_str(old).expect("an old message still loads");
    assert_eq!(legacy.status_line, None);
    serde_json::from_str(&json).expect("deserialises")
}

#[test]
fn the_session_row_counts_and_names_the_session() {
    assert_eq!(message_count_label(1), "1 message");
    assert_eq!(message_count_label(0), "0 messages");
    assert_eq!(message_count_label(3), "3 messages");
    assert_eq!(session_summary(&[]), "🟢 New session");
    let short = [
        ChatMessage::assistant("a", "hi"),
        ChatMessage::user("u", "two\nlines"),
    ];
    assert_eq!(session_summary(&short), "• two lines");
    let long = [ChatMessage::user(
        "u",
        "abcdefghijklmnopqrstuvwxyz0123456789",
    )];
    assert_eq!(session_summary(&long), "• abcdefghijklmnopqrstuvwxyz01...");
}

#[test]
fn a_long_code_block_scrolls_a_short_one_does_not() {
    let lines = |n: usize| vec!["x"; n].join("\n");
    assert!(!code_block_scrolls(&lines(CODE_BLOCK_MAX_LINES)));
    assert!(code_block_scrolls(&lines(CODE_BLOCK_MAX_LINES + 1)));
    assert!(!code_block_scrolls(""));
}

#[test]
fn cli_fallback_notice_short_reason() {
    let notice = cli_fallback_notice("connection refused");
    assert_eq!(
        notice,
        "ACP unavailable — answered through the CLI (connection refused)"
    );
}

#[test]
fn cli_fallback_notice_multi_line_reason() {
    let notice = cli_fallback_notice("first line of error\nsecond line details\nthird line stack");
    assert_eq!(
        notice,
        "ACP unavailable — answered through the CLI (first line of error)"
    );
}

#[test]
fn cli_fallback_notice_300_char_reason() {
    let long_reason = "x".repeat(300);
    let notice = cli_fallback_notice(&long_reason);
    let expected_inner = format!("{}…", "x".repeat(120));
    assert_eq!(
        notice,
        format!("ACP unavailable — answered through the CLI ({expected_inner})")
    );
}

#[test]
fn cli_fallback_notice_multibyte_reason() {
    // Multi-byte Unicode characters (emoji and accented characters)
    let multibyte_str = "🦀".repeat(150);
    let notice = cli_fallback_notice(&multibyte_str);
    let expected_inner = format!("{}…", "🦀".repeat(120));
    assert_eq!(
        notice,
        format!("ACP unavailable — answered through the CLI ({expected_inner})")
    );
}

#[test]
fn turns_json_includes_notice_field() {
    let u = ChatMessage::user("u1", "Help");
    let mut a = ChatMessage::assistant("a1", "Sure");
    a.notice = Some("ACP unavailable — answered through the CLI (timeout)".to_string());

    let val = turns_json(&[u, a], "Copilot", &crate::acp_live::LiveStatus::Idle, None);
    let turns_array = val["turns"].as_array().expect("turns array");
    assert_eq!(
        turns_array[0]["reply"]["notice"],
        "ACP unavailable — answered through the CLI (timeout)"
    );
    assert_eq!(val["agent"]["live"], "idle");
    assert_eq!(val["agent"]["session"], serde_json::Value::Null);
}

#[test]
fn the_chip_says_who_answered_and_how() {
    use crate::acp_live::LiveStatus;
    use crate::session::{ChatMessage, MessageStatus};
    let idle = LiveStatus::Idle;
    let user = ChatMessage::user("u", "hi");
    assert_eq!(
        agent_chip(std::slice::from_ref(&user), "Claude", &idle).link,
        ChipLink::Waiting
    );

    let mut acp = ChatMessage::assistant("a", "hello");
    acp.acp_session = Some("3f2a9c01-77aa-4bbb".into());
    let chip = agent_chip(&[user.clone(), acp.clone()], "Claude", &idle);
    assert_eq!(chip.text, "Claude · ACP · session 3f2a9c01");
    assert_eq!(chip.link, ChipLink::Acp);

    let mut cli = ChatMessage::assistant("b", "hello");
    cli.notice = Some("ACP unavailable — answered through the CLI (x)".into());
    let chip = agent_chip(&[user.clone(), acp.clone(), cli], "Claude", &idle);
    assert_eq!(
        (chip.text.as_str(), chip.link),
        ("Claude · CLI (ACP unavailable)", ChipLink::Cli)
    );

    let mut failed = ChatMessage::assistant("c", "⚠️ Agent error: gone");
    failed.status = MessageStatus::Error;
    assert_eq!(
        agent_chip(&[user.clone(), failed], "Claude", &idle).link,
        ChipLink::Failed
    );

    // a "thinking" placeholder is not an answer: the chip keeps the last one
    let thinking = ChatMessage::thinking("t", "…");
    assert_eq!(
        agent_chip(&[user, acp, thinking], "Claude", &idle).link,
        ChipLink::Acp
    );
}

#[test]
fn the_chip_shows_live_status_before_first_answer() {
    use crate::acp_live::LiveStatus;
    use crate::session::ChatMessage;
    let user = ChatMessage::user("u", "hi");
    let msgs = [user];

    let connecting = LiveStatus::Connecting;
    let chip_c = agent_chip(&msgs, "Claude", &connecting);
    assert_eq!(chip_c.text, "Claude · connecting…");
    assert_eq!(chip_c.link, ChipLink::Waiting);

    let ready = LiveStatus::Ready {
        session: "3f2a9c01-long-uuid".to_string(),
    };
    let chip_r = agent_chip(&msgs, "Claude", &ready);
    assert_eq!(chip_r.text, "Claude · ACP · session 3f2a9c01");
    assert_eq!(chip_r.link, ChipLink::Acp);

    let failed = LiveStatus::Failed("Process exited 1".to_string());
    let chip_f = agent_chip(&msgs, "Claude", &failed);
    assert_eq!(chip_f.text, "Claude · ACP unavailable");
    assert_eq!(chip_f.link, ChipLink::Cli);

    let idle = LiveStatus::Idle;
    let chip_i = agent_chip(&msgs, "Claude", &idle);
    assert_eq!(chip_i.text, "Claude · waiting for the first answer");
    assert_eq!(chip_i.link, ChipLink::Waiting);
}

#[test]
fn reply_segments_with_three_code_blocks_and_prose_between() {
    let reply = "First intro sentence.\n```rust\nfn one() {}\n```\nMiddle sentence.\n```rust\nfn two() {}\n```\nAnother middle sentence.\n```toml\n[package]\n```\nClosing sentence.";
    let segments = reply_segments(reply);
    assert_eq!(segments.len(), 7);
    assert_eq!(
        segments[0],
        ReplySegment::Prose("First intro sentence.".into())
    );
    assert_eq!(
        segments[1],
        ReplySegment::Code {
            language: "rust".into(),
            code: "fn one() {}".into(),
        }
    );
    assert_eq!(segments[2], ReplySegment::Prose("Middle sentence.".into()));
    assert_eq!(
        segments[3],
        ReplySegment::Code {
            language: "rust".into(),
            code: "fn two() {}".into(),
        }
    );
    assert_eq!(
        segments[4],
        ReplySegment::Prose("Another middle sentence.".into())
    );
    assert_eq!(
        segments[5],
        ReplySegment::Code {
            language: "toml".into(),
            code: "[package]".into(),
        }
    );
    assert_eq!(segments[6], ReplySegment::Prose("Closing sentence.".into()));
}

#[test]
fn reply_segments_with_one_code_block() {
    let reply = "Here is the code:\n```rust\nlet x = 42;\n```";
    let segments = reply_segments(reply);
    assert_eq!(segments.len(), 2);
    assert_eq!(segments[0], ReplySegment::Prose("Here is the code:".into()));
    assert_eq!(
        segments[1],
        ReplySegment::Code {
            language: "rust".into(),
            code: "let x = 42;".into(),
        }
    );
}

#[test]
fn reply_segments_with_no_fence_returns_single_prose() {
    let reply = "Just plain prose without any code fence.";
    let segments = reply_segments(reply);
    assert_eq!(segments.len(), 1);
    assert_eq!(
        segments[0],
        ReplySegment::Prose("Just plain prose without any code fence.".into())
    );
}

#[test]
fn reply_segments_merges_adjacent_text_blocks() {
    let reply = "First paragraph.\n\nSecond paragraph.";
    let segments = reply_segments(reply);
    assert_eq!(segments.len(), 1);
    assert_eq!(
        segments[0],
        ReplySegment::Prose("First paragraph.\n\nSecond paragraph.".into())
    );
}

#[test]
fn reply_segments_preserves_language_or_defaults_to_code() {
    let reply = "```\nno lang\n```\n```rust\nwith lang\n```";
    let segments = reply_segments(reply);
    assert_eq!(segments.len(), 2);
    assert_eq!(
        segments[0],
        ReplySegment::Code {
            language: "code".into(),
            code: "no lang".into(),
        }
    );
    assert_eq!(
        segments[1],
        ReplySegment::Code {
            language: "rust".into(),
            code: "with lang".into(),
        }
    );
}

#[test]
fn turns_json_includes_blocks_in_reply() {
    use crate::acp_live::LiveStatus;
    use crate::session::ChatMessage;

    let user = ChatMessage::user("u1", "Give me code");
    let assistant = ChatMessage::assistant("a1", "Intro\n```rust\nlet y = 10;\n```\nOutro");
    let messages = [user, assistant];

    let json = turns_json(&messages, "Copilot", &LiveStatus::Idle, None);
    let turns = json.get("turns").and_then(|v| v.as_array()).unwrap();
    assert_eq!(turns.len(), 1);

    let reply = turns[0].get("reply").unwrap();
    let blocks = reply.get("blocks").and_then(|v| v.as_array()).unwrap();
    assert_eq!(blocks.len(), 3);
    assert_eq!(blocks[0]["kind"], "prose");
    assert_eq!(blocks[0]["text"], "Intro");
    assert_eq!(blocks[1]["kind"], "code");
    assert_eq!(blocks[1]["language"], "rust");
    assert_eq!(blocks[1]["text"], "let y = 10;");
    assert_eq!(blocks[2]["kind"], "prose");
    assert_eq!(blocks[2]["text"], "Outro");
}

#[test]
fn popup_margins_calculations() {
    let (cx, cy) = context_popup_margin((20.0, 500.0), (10.0, 500.0), (0.0, 0.0, 600.0), 200.0);
    assert_eq!((cx, cy), (18.0, 296.0));

    let (sx, sy) = slash_popup_margin((10.0, 500.0), (0.0, 0.0, 600.0), 200.0);
    assert_eq!((sx, sy), (10.0, 296.0));

    let (fx, fy) = slash_popup_margin((0.0, 0.0), (0.0, 0.0, 600.0), 200.0);
    assert_eq!((fx, fy), (8.0, 315.0));
}
