//! Layout calculations and turn representations for chat message bubbles and cards.

use crawrad_note::note;
#[note("chat-layout-user-bubble-min-indent")]
pub const USER_BUBBLE_MIN_INDENT: f64 = 24.0;

#[note("chat-layout-user-bubble-indent")]
pub fn user_bubble_indent(text: &str, meta: &str, available_width: f64) -> f64 {
    let longest_line = text.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    let meta_len = meta.chars().count();
    let estimated = longest_line as f64 * 6.1 + 10.0 + meta_len as f64 * 5.5 + 24.0 + 36.0;
    let max_width = (available_width - USER_BUBBLE_MIN_INDENT).max(80.0);
    if estimated >= max_width {
        USER_BUBBLE_MIN_INDENT
    } else {
        (available_width - estimated).max(USER_BUBBLE_MIN_INDENT)
    }
}

#[note("chat-layout-code-block-max-lines")]
pub const CODE_BLOCK_MAX_LINES: usize = 8;

#[note("chat-layout-code-block-scrolls")]
pub fn code_block_scrolls(code: &str) -> bool {
    code.lines().count() > CODE_BLOCK_MAX_LINES
}

#[note("chat-layout-delivery-mark")]
pub fn delivery_mark(reply_started: bool) -> &'static str {
    if reply_started {
        "\u{ebb1}"
    } else {
        "\u{eab2}"
    }
}

#[note("chat-layout-delivery-word")]
pub fn delivery_word(reply_started: bool) -> &'static str {
    if reply_started { "✓✓" } else { "✓" }
}

#[note("chat-layout-cli-fallback-notice")]
pub fn cli_fallback_notice(reason: &str) -> String {
    let first_line = reason.lines().next().unwrap_or("").trim();
    let char_count = first_line.chars().count();
    let formatted_reason = if char_count > 120 {
        let truncated: String = first_line.chars().take(120).collect();
        format!("{truncated}…")
    } else {
        first_line.to_string()
    };
    format!("ACP unavailable — answered through the CLI ({formatted_reason})")
}

#[note("chat-layout-replysegment")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplySegment {
    /// A regular prose text block.
    Prose(String),
    /// A fenced code block with language and source content.
    Code { language: String, code: String },
}

impl ReplySegment {
    #[note("chat-layout-replysegment-view")]
    pub fn view(&self) -> (Option<&str>, &str, bool) {
        match self {
            Self::Prose(text) => (None, text, false),
            Self::Code { language, code } => (Some(language), code, code_block_scrolls(code)),
        }
    }
}

#[note("chat-layout-reply-segments")]
pub fn reply_segments(reply_text: &str) -> Vec<ReplySegment> {
    let blocks = crate::markdown::parse_chat_markdown(reply_text);
    let mut segments = Vec::new();
    let mut pending_prose = String::new();

    for block in blocks {
        match block {
            crate::markdown::ChatContentBlock::Text(t) => {
                if !pending_prose.is_empty() {
                    pending_prose.push_str("\n\n");
                }
                pending_prose.push_str(&t);
            }
            crate::markdown::ChatContentBlock::CodeBlock { language, code } => {
                if !pending_prose.is_empty() {
                    segments.push(ReplySegment::Prose(std::mem::take(&mut pending_prose)));
                }
                let lang = if language.is_empty() {
                    "code".to_string()
                } else {
                    language
                };
                segments.push(ReplySegment::Code {
                    language: lang,
                    code,
                });
            }
        }
    }

    if !pending_prose.is_empty() {
        segments.push(ReplySegment::Prose(pending_prose));
    }

    if segments.is_empty() {
        segments.push(ReplySegment::Prose(reply_text.to_string()));
    }

    segments
}

#[note("chat-layout-blockrect")]
pub type BlockRect<'a> = (&'a str, [f64; 4], Option<[f64; 4]>);

#[note("chat-layout-card-json")]
pub fn card_json(
    text_rect: [f64; 4],
    copy_rect: serde_json::Value,
    selection: &str,
    blocks: &[BlockRect<'_>],
) -> serde_json::Value {
    let blocks: Vec<_> = blocks
        .iter()
        .map(|(kind, rect, copy)| match copy {
            Some(c) => serde_json::json!({"kind": kind, "rect": rect, "copy_rect": c}),
            None => serde_json::json!({"kind": kind, "rect": rect}),
        })
        .collect();
    serde_json::json!({"text_rect": text_rect, "copy_rect": copy_rect, "selection": selection, "block_rects": blocks})
}

#[note("chat-layout-chatturn")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatTurn {
    pub user_text: Option<String>,
    pub user_sent_at: Option<String>,
    pub user_meta: Option<String>,
    pub user_mark: Option<&'static str>,
    pub reply_text: Option<String>,
    pub reply_segments: Vec<ReplySegment>,
    pub reply_sent_at: Option<String>,
    pub reply_meta: Option<String>,
    pub sender_name: Option<String>,
    pub tool_calls: Vec<crate::markdown::ToolCallRecord>,
    pub notice: Option<String>,
    /// The reply's final status line, when the turn recorded one.
    pub status_line: Option<crate::acp_status::StatusLine>,
    pub has_reply: bool,
}

#[note("chat-layout-copy-message")]
pub fn copy_message(
    messages: &[crate::session::ChatMessage],
    default_sender: &str,
    args: &serde_json::Value,
) -> Result<String, String> {
    let turn = args.get("turn").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let part = args.get("part").and_then(|v| v.as_str()).unwrap_or("reply");
    let found = group_chat_turns(messages, default_sender)
        .into_iter()
        .nth(turn);
    let text = found.and_then(|t| {
        if part == "user" {
            t.user_text
        } else {
            t.reply_text
        }
    });
    text.ok_or_else(|| format!("No {part} at turn {turn}"))
}

#[note("chat-layout-chat-controls")]
pub const CHAT_CONTROLS: [&str; 20] = [
    "btn_ai_new",
    "btn_ai_clear",
    "btn_ai_history",
    "btn_ai_settings",
    "btn_form_toggle",
    "btn_sel_toggle",
    "btn_ai_attach",
    "btn_prompt_send",
    "btn_prompt_stop",
    "btn_ai_mic",
    "btn_ai_media",
    "btn_ai_mentions",
    "btn_ai_actions",
    "btn_ai_memory",
    "btn_ai_mcp",
    "btn_ai_agent",
    "btn_perm_allow_once",
    "btn_perm_allow_always",
    "btn_perm_reject_once",
    "btn_perm_reject_always",
];

#[note("chat-layout-context-switches")]
pub const CONTEXT_SWITCHES: [(&str, &str); 2] =
    [("btn_form_toggle", "Form"), ("btn_sel_toggle", "Sel")];

#[note("chat-layout-context-items")]
pub const CONTEXT_ITEMS: [&str; 6] = ["media", "mentions", "actions", "memory", "mcp", "agent"];

#[note("chat-layout-hero-shortcuts")]
pub const HERO_SHORTCUTS: [(&str, &str); 7] = [
    ("explain", "/explain "),
    ("refactor", "/refactor "),
    ("test", "/test "),
    ("document", "/docs "),
    ("find", "/find "),
    ("optimize", "/optimize "),
    ("teach", "/teach "),
];

#[note("chat-layout-permission-choices")]
pub const PERMISSION_CHOICES: [&str; 4] =
    ["allow_once", "allow_always", "reject_once", "reject_always"];

#[note("chat-layout-append-token")]
pub fn append_token(text: &str, token: &str) -> String {
    let sep = if text.is_empty() || text.ends_with(' ') {
        ""
    } else {
        " "
    };
    format!("{text}{sep}{token}")
}

#[note("chat-layout-mention-token")]
pub fn mention_token(active: Option<&std::path::Path>, root: Option<&std::path::Path>) -> String {
    let Some(path) = active else {
        return "@".into();
    };
    let rel = root.and_then(|r| path.strip_prefix(r).ok()).unwrap_or(path);
    format!("@{} ", rel.to_string_lossy().replace('\\', "/"))
}

#[note("chat-layout-permission-json")]
pub fn permission_json(queue: &crate::permission::PermissionQueue) -> serde_json::Value {
    queue.peek().map_or(serde_json::Value::Null, |r| {
        serde_json::json!({"tool": r.tool_name, "target": r.target, "arguments": r.arguments_summary, "waiting": queue.len()})
    })
}

#[note("chat-layout-switch-label")]
pub fn switch_label(name: &str, on: bool) -> String {
    format!("{name}: {}", if on { "ON" } else { "OFF" })
}

pub fn group_chat_turns(
    messages: &[crate::session::ChatMessage],
    default_sender: &str,
) -> Vec<ChatTurn> {
    let mut turns: Vec<ChatTurn> = Vec::new();
    for msg in messages {
        match msg.sender {
            crate::session::MessageSender::User => {
                turns.push(ChatTurn {
                    user_text: Some(msg.text.clone()),
                    user_sent_at: msg.sent_at.clone(),
                    user_meta: None,
                    user_mark: None,
                    reply_text: None,
                    reply_segments: Vec::new(),
                    reply_sent_at: None,
                    reply_meta: None,
                    sender_name: None,
                    tool_calls: Vec::new(),
                    notice: None,
                    status_line: None,
                    has_reply: false,
                });
            }
            crate::session::MessageSender::Assistant => {
                let segments = reply_segments(&msg.text);
                if let Some(last) = turns.last_mut()
                    && !last.has_reply
                {
                    last.reply_text = Some(msg.text.clone());
                    last.reply_segments = segments;
                    last.reply_sent_at = msg.sent_at.clone();
                    last.reply_meta = msg.sent_at.clone();
                    last.has_reply = true;
                    last.sender_name = Some(default_sender.to_string());
                    last.tool_calls = msg.tool_calls.clone();
                    last.notice = msg.notice.clone();
                    last.status_line = msg.status_line.clone();
                    continue;
                }
                turns.push(ChatTurn {
                    user_text: None,
                    user_sent_at: None,
                    user_meta: None,
                    user_mark: None,
                    reply_text: Some(msg.text.clone()),
                    reply_segments: segments,
                    reply_sent_at: msg.sent_at.clone(),
                    reply_meta: msg.sent_at.clone(),
                    has_reply: true,
                    sender_name: Some(default_sender.to_string()),
                    tool_calls: msg.tool_calls.clone(),
                    notice: msg.notice.clone(),
                    status_line: msg.status_line.clone(),
                });
            }
            _ => {}
        }
    }

    for turn in &mut turns {
        if turn.user_text.is_some() {
            turn.user_mark = Some(delivery_mark(turn.has_reply));
            turn.user_meta = turn.user_sent_at.clone();
        }
    }

    turns
}

#[note("chat-layout-user-meta-text")]
pub fn user_meta_text(turn: &ChatTurn) -> String {
    let mark = delivery_word(turn.has_reply);
    match &turn.user_sent_at {
        Some(time) => format!("{time} {mark}"),
        None => mark.to_string(),
    }
}

#[note("chat-layout-turns-json")]
pub fn turns_json(
    messages: &[crate::session::ChatMessage],
    default_sender: &str,
    live: &crate::acp_live::LiveStatus,
    status: Option<&crate::acp_status::StatusLine>,
) -> serde_json::Value {
    let turns = group_chat_turns(messages, default_sender);
    let mut turns_val = Vec::new();

    for turn in turns {
        let meta = user_meta_text(&turn);
        let user = turn.user_text.map(|text| {
            serde_json::json!({
                "text": text,
                "meta": meta,
            })
        });

        let reply = turn.reply_text.map(|_| {
            let meta = turn.reply_meta.unwrap_or_default();
            let mut prose_buf = String::new();
            let blocks: Vec<_> = turn
                .reply_segments
                .iter()
                .map(|seg| match seg {
                    ReplySegment::Prose(t) => {
                        if !prose_buf.is_empty() {
                            prose_buf.push_str("\n\n");
                        }
                        prose_buf.push_str(t);
                        serde_json::json!({"kind": "prose", "text": t})
                    }
                    ReplySegment::Code { language, code } => {
                        serde_json::json!({"kind": "code", "language": language, "text": code})
                    }
                })
                .collect();
            serde_json::json!({
                "text": prose_buf,
                "meta": meta,
                "notice": turn.notice,
                "status": turn.status_line,
                "blocks": blocks,
            })
        });

        turns_val.push(serde_json::json!({
            "user": user,
            "reply": reply,
        }));
    }

    let chip = agent_chip(messages, default_sender, live);
    let (live_str, session_val) = match live {
        crate::acp_live::LiveStatus::Idle => ("idle", serde_json::Value::Null),
        crate::acp_live::LiveStatus::Connecting => ("connecting", serde_json::Value::Null),
        crate::acp_live::LiveStatus::Ready { session } => {
            ("ready", serde_json::Value::String(session.clone()))
        }
        crate::acp_live::LiveStatus::Failed(_) => ("failed", serde_json::Value::Null),
    };
    // Why the agent did not connect, readable from outside.
    let failure = match live {
        crate::acp_live::LiveStatus::Failed(reason) => serde_json::Value::String(reason.clone()),
        _ => serde_json::Value::Null,
    };
    serde_json::json!({
        "turns": turns_val,
        "status": status,
        "agent": {
            "text": chip.text,
            "link": chip.link.as_str(),
            "live": live_str,
            "session": session_val,
            "failure": failure,
        },
    })
}

#[note("chat-layout-via")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Via {
    Acp { session: Option<String> },
    Cli { notice: String },
}

#[note("chat-layout-chiplink")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipLink {
    /// No answer yet in this conversation.
    Waiting,
    /// The last answer came through ACP.
    Acp,
    /// The last answer came through the CLI fallback.
    Cli,
    /// The last turn failed.
    Failed,
}

impl ChipLink {
    #[note("chat-layout-chiplink-as-str")]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Waiting => "waiting",
            Self::Acp => "acp",
            Self::Cli => "cli",
            Self::Failed => "failed",
        }
    }

    #[note("chat-layout-chiplink-dot")]
    pub fn dot(self) -> &'static str {
        match self {
            Self::Waiting => "chip_dot_waiting",
            Self::Acp => "chip_dot_acp",
            Self::Cli => "chip_dot_cli",
            Self::Failed => "chip_dot_failed",
        }
    }
}

#[note("chat-layout-chip-dots")]
pub const CHIP_DOTS: [&str; 4] = [
    "chip_dot_waiting",
    "chip_dot_acp",
    "chip_dot_cli",
    "chip_dot_failed",
];

#[note("chat-layout-agentchip")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentChip {
    pub text: String,
    pub link: ChipLink,
}

#[note("chat-layout-agent-chip")]
pub fn agent_chip(
    messages: &[crate::session::ChatMessage],
    agent: &str,
    live: &crate::acp_live::LiveStatus,
) -> AgentChip {
    use crate::acp_live::LiveStatus;
    use crate::session::{MessageSender, MessageStatus};
    let answered = |m: &&crate::session::ChatMessage| {
        m.sender == MessageSender::Assistant && m.status != MessageStatus::Thinking
    };
    let (text, link) = match messages.iter().rev().find(answered) {
        None => match live {
            LiveStatus::Connecting => (format!("{agent} · connecting\u{2026}"), ChipLink::Waiting),
            LiveStatus::Ready { session } => {
                let short: String = session.chars().take(8).collect();
                (format!("{agent} · ACP · session {short}"), ChipLink::Acp)
            }
            LiveStatus::Failed(_) => (format!("{agent} · ACP unavailable"), ChipLink::Cli),
            LiveStatus::Idle => (
                format!("{agent} · waiting for the first answer"),
                ChipLink::Waiting,
            ),
        },
        Some(m) if m.status == MessageStatus::Error => {
            (format!("{agent} · error"), ChipLink::Failed)
        }
        Some(m) if m.notice.is_some() => {
            (format!("{agent} · CLI (ACP unavailable)"), ChipLink::Cli)
        }
        Some(m) => {
            let session = m.acp_session.as_deref().map(|s| {
                let short: String = s.chars().take(8).collect();
                format!(" · session {short}")
            });
            (
                format!("{agent} · ACP{}", session.unwrap_or_default()),
                ChipLink::Acp,
            )
        }
    };
    AgentChip { text, link }
}

/// The three texts of the empty-chat hero, by label id: message count, session summary and the
/// real state of the AI agent (never a fixed "connected").
pub fn hero_labels(
    messages: &[crate::session::ChatMessage],
    agent: crate::acp_registry::AgentState,
    connected: bool,
) -> [(&'static str, String); 3] {
    [
        ("lbl_session_count", message_count_label(messages.len())),
        ("lbl_session_status", session_summary(messages)),
        (
            "lbl_ready_status",
            crate::acp_registry::chat_status_line(agent, connected).to_string(),
        ),
    ]
}

#[note("chat-layout-message-count-label")]
pub fn message_count_label(count: usize) -> String {
    match count {
        1 => "1 message".to_string(),
        n => format!("{n} messages"),
    }
}

#[note("chat-layout-session-summary")]
pub fn session_summary(messages: &[crate::session::ChatMessage]) -> String {
    let first = messages
        .iter()
        .find(|m| m.sender == crate::session::MessageSender::User);
    let Some(first) = first else {
        return "🟢 New session".to_string();
    };
    let one_line = first.text.replace('\n', " ");
    if first.text.chars().count() > 28 {
        let cut: String = first.text.chars().take(28).collect();
        format!("• {}...", cut.replace('\n', " "))
    } else {
        format!("• {one_line}")
    }
}

#[note("chat-layout-context-popup-margin")]
pub fn context_popup_margin(
    (btn_x, btn_y): (f64, f64),
    (bar_x, bar_y): (f64, f64),
    (panel_x, panel_y, panel_h): (f64, f64, f64),
    popup_h: f64,
) -> (f64, f64) {
    if bar_y > 0.0 && panel_h > 0.0 {
        let rel_x = if btn_x > 0.0 {
            (btn_x - panel_x - 2.0).max(8.0)
        } else {
            (bar_x - panel_x).max(8.0)
        };
        let bar_rel_y = bar_y - panel_y;
        let rel_y = (bar_rel_y - popup_h - 4.0).max(4.0);
        (rel_x, rel_y)
    } else if btn_y > 0.0 && panel_h > 0.0 {
        let rel_x = (btn_x - panel_x - 2.0).max(8.0);
        let btn_rel_y = btn_y - panel_y;
        let rel_y = (btn_rel_y - popup_h - 10.0).max(4.0);
        (rel_x, rel_y)
    } else {
        let fallback_y = (panel_h - popup_h - 85.0).max(4.0);
        (14.0, fallback_y)
    }
}

#[note("chat-layout-slash-popup-margin")]
pub fn slash_popup_margin(
    (bar_x, bar_y): (f64, f64),
    (panel_x, panel_y, panel_h): (f64, f64, f64),
    popup_h: f64,
) -> (f64, f64) {
    if bar_y > 0.0 && panel_h > 0.0 {
        let rel_x = (bar_x - panel_x).max(8.0);
        let bar_rel_y = bar_y - panel_y;
        let rel_y = (bar_rel_y - popup_h - 4.0).max(4.0);
        (rel_x, rel_y)
    } else {
        let fallback_y = (panel_h - popup_h - 85.0).max(4.0);
        (8.0, fallback_y)
    }
}

#[cfg(test)]
#[path = "test/chat_layout/test.rs"]
mod tests;
