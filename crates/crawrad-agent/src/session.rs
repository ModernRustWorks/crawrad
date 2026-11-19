use super::agent_profile::SessionSupport;
use crate::chat_attachment::SelectionAttachment;
use crawrad_note::note;
use serde::{Deserialize, Serialize};

#[note("session-messagesender")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageSender {
    User,
    Assistant,
    System,
}

#[note("session-planstatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    Pending,
    Approved,
    AutoApproved,
    Rejected,
}

impl PlanStatus {
    pub fn badge_label(&self) -> &'static str {
        match self {
            Self::Pending => "Pending Review",
            Self::Approved => "🟢 Approved",
            Self::AutoApproved => "🟢 Auto-Approved",
            Self::Rejected => "🔴 Rejected",
        }
    }
}

#[note("session-messagestatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    Sending,
    Thinking,
    Streaming,
    Complete,
    Error,
}

#[note("session-chatmessage")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub sender: MessageSender,
    pub text: String,
    pub status: MessageStatus,
    #[serde(default)]
    pub sent_at: Option<String>,
    #[serde(default)]
    pub thought: Option<String>,
    #[serde(default)]
    pub tool_calls: Vec<super::markdown::ToolCallRecord>,
    #[serde(default)]
    pub notice: Option<String>,
    /// The ACP session that answered, when the reply came through ACP.
    #[serde(default)]
    pub acp_session: Option<String>,
    /// The turn's final status line, drawn dimmed at the top of the card.
    #[serde(default)]
    pub status_line: Option<super::acp_status::StatusLine>,
}

impl ChatMessage {
    pub fn user(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            sender: MessageSender::User,
            text: text.into(),
            status: MessageStatus::Complete,
            sent_at: Some(crawrad_os::clock::local_hh_mm()),
            thought: None,
            tool_calls: Vec::new(),
            notice: None,
            acp_session: None,
            status_line: None,
        }
    }

    pub fn assistant(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            sender: MessageSender::Assistant,
            text: text.into(),
            status: MessageStatus::Complete,
            sent_at: Some(crawrad_os::clock::local_hh_mm()),
            thought: None,
            tool_calls: Vec::new(),
            notice: None,
            acp_session: None,
            status_line: None,
        }
    }

    pub fn thinking(id: impl Into<String>, thought: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            sender: MessageSender::Assistant,
            text: String::new(),
            status: MessageStatus::Thinking,
            sent_at: Some(crawrad_os::clock::local_hh_mm()),
            thought: Some(thought.into()),
            tool_calls: Vec::new(),
            notice: None,
            acp_session: None,
            status_line: None,
        }
    }

    pub fn with_tool_calls(mut self, calls: Vec<super::markdown::ToolCallRecord>) -> Self {
        self.tool_calls = calls;
        self
    }

    pub fn with_status_line(mut self, status: Option<super::acp_status::StatusLine>) -> Self {
        self.status_line = status;
        self
    }

    pub fn with_notice(mut self, notice: Option<String>) -> Self {
        self.notice = notice;
        self
    }

    pub fn add_tool_call(&mut self, call: super::markdown::ToolCallRecord) {
        self.tool_calls.push(call);
    }
}

#[note("session-turnoutcome")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnOutcome {
    /// Agent executed successfully and returned response text.
    Success { reply: String },
    /// Session was lost or expired on the agent side, resetting session state and notifying the user.
    SessionLost { error: String, notice: String },
    /// Agent process execution or network error occurred.
    Error { error: String },
}

impl TurnOutcome {
    #[note("session-turnoutcome-reply-text")]
    pub fn reply_text(self) -> String {
        match self {
            Self::Success { reply } => reply,
            Self::SessionLost { error, notice } => {
                format!("⚠️ Agent error: {error}\n\nℹ️ {notice}")
            }
            Self::Error { error } => format!("⚠️ Agent error: {error}"),
        }
    }
}

#[note("session-chatsession")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub title: String,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub cli_session_id: Option<String>,
    #[serde(default)]
    pub session_started: bool,
    #[serde(default)]
    pub active_project_dir: Option<std::path::PathBuf>,
}

impl Default for ChatSession {
    fn default() -> Self {
        Self {
            id: "session-default".into(),
            title: "New session".into(),
            model: "auto".into(),
            messages: Vec::new(),
            cli_session_id: None,
            session_started: false,
            active_project_dir: None,
        }
    }
}

impl ChatSession {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            model: "auto".into(),
            messages: Vec::new(),
            cli_session_id: None,
            session_started: false,
            active_project_dir: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.cli_session_id = None;
        self.session_started = false;
    }

    #[note("session-chatsession-ensure-cli-session-id")]
    pub fn ensure_cli_session_id(&mut self) -> &str {
        if self.cli_session_id.is_none() {
            self.cli_session_id = Some(uuid::Uuid::new_v4().to_string());
            self.session_started = false;
        }
        self.cli_session_id.as_deref().unwrap()
    }

    #[note("session-chatsession-mark-session-started")]
    pub fn mark_session_started(&mut self) {
        self.session_started = true;
    }

    #[note("session-chatsession-reset-cli-session")]
    pub fn reset_cli_session(&mut self) {
        self.cli_session_id = None;
        self.session_started = false;
    }

    #[note("session-chatsession-reset-if-project-changed")]
    pub fn reset_if_project_changed(&mut self, new_project_dir: Option<&std::path::Path>) -> bool {
        let changed = self.active_project_dir.as_deref() != new_project_dir;
        if changed {
            self.active_project_dir = new_project_dir.map(std::path::PathBuf::from);
            self.reset_cli_session();
        }
        changed
    }

    #[note("session-chatsession-extract-captured-conversation-id")]
    pub fn extract_captured_conversation_id(output: &str) -> Option<String> {
        // Parse event init line: {"event":"init","conversation_id":"..."}
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.contains("\"conversation_id\"")
                && let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed)
                && let Some(id) = v.get("conversation_id").and_then(|c| c.as_str())
                && !id.trim().is_empty()
            {
                return Some(id.trim().to_string());
            }
        }
        None
    }

    #[note("session-chatsession-apply-turn-result")]
    pub fn apply_turn_result(&mut self, result: &Result<String, String>) -> TurnOutcome {
        self.apply_turn_result_for_support(result, SessionSupport::Pinned)
    }

    #[note("session-chatsession-apply-turn-result-for-support")]
    pub fn apply_turn_result_for_support(
        &mut self,
        result: &Result<String, String>,
        support: SessionSupport,
    ) -> TurnOutcome {
        match result {
            Ok(output) => {
                let trimmed = output.trim();
                let output_lower = trimmed.to_lowercase();

                let (final_reply, session_was_reset) = match support {
                    SessionSupport::Captured => {
                        // Check if output is JSONL (e.g. from OpenAI Codex CLI)
                        let mut codex_thread_id = None;
                        let mut codex_messages = Vec::new();
                        let mut codex_error = None;
                        let mut is_jsonl = false;

                        for line in trimmed.lines() {
                            let l = line.trim();
                            if l.starts_with('{')
                                && l.ends_with('}')
                                && let Ok(val) = serde_json::from_str::<serde_json::Value>(l)
                            {
                                match val.get("type").and_then(|t| t.as_str()) {
                                    Some("thread.started") => {
                                        is_jsonl = true;
                                        if let Some(tid) =
                                            val.get("thread_id").and_then(|id| id.as_str())
                                        {
                                            codex_thread_id = Some(tid.to_string());
                                        }
                                    }
                                    Some("item.completed") => {
                                        is_jsonl = true;
                                        if let Some(item) = val.get("item")
                                            && item.get("type").and_then(|t| t.as_str())
                                                == Some("agent_message")
                                            && let Some(text) =
                                                item.get("text").and_then(|t| t.as_str())
                                        {
                                            codex_messages.push(text.to_string());
                                        }
                                    }
                                    Some("error") | Some("turn.failed") => {
                                        is_jsonl = true;
                                        if let Some(err_val) =
                                            val.get("message").or_else(|| val.get("error"))
                                        {
                                            let msg = if let Some(s) = err_val.as_str() {
                                                if let Ok(inner) =
                                                    serde_json::from_str::<serde_json::Value>(s)
                                                {
                                                    inner
                                                        .get("error")
                                                        .and_then(|e| e.get("message"))
                                                        .and_then(|m| m.as_str())
                                                        .map(|m| m.to_string())
                                                        .unwrap_or_else(|| s.to_string())
                                                } else {
                                                    s.to_string()
                                                }
                                            } else {
                                                err_val.to_string()
                                            };
                                            codex_error = Some(msg);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        if is_jsonl {
                            if let Some(err) = codex_error {
                                return TurnOutcome::Error { error: err };
                            }
                            if !codex_messages.is_empty() {
                                if let Some(tid) = codex_thread_id {
                                    self.cli_session_id = Some(tid);
                                }
                                self.mark_session_started();
                                return TurnOutcome::Success {
                                    reply: codex_messages.join("\n\n"),
                                };
                            }
                        }

                        let json_str = if let Some(idx) = trimmed.find('{') {
                            &trimmed[idx..]
                        } else {
                            trimmed
                        };

                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                            if let Some(status) = json.get("status").and_then(|s| s.as_str())
                                && status != "SUCCESS"
                            {
                                let err =
                                    json.get("error").and_then(|e| e.as_str()).unwrap_or(status);
                                return TurnOutcome::Error {
                                    error: format!("Agent returned status `{status}`: {err}"),
                                };
                            }

                            let old_id = self.cli_session_id.clone();
                            let new_id = json
                                .get("conversation_id")
                                .and_then(|c| c.as_str())
                                .map(|s| s.trim().to_string());
                            let resp_text = json
                                .get("response")
                                .and_then(|r| r.as_str())
                                .map(|s| s.trim().to_string())
                                .unwrap_or_else(|| trimmed.to_string());

                            let id_changed = match (&old_id, &new_id) {
                                (Some(old), Some(new)) => old != new,
                                _ => false,
                            };
                            let has_warning = output_lower.contains("warning: conversation")
                                && output_lower.contains("not found");
                            let lost = (id_changed || has_warning) && self.session_started;

                            if let Some(id) = new_id {
                                self.cli_session_id = Some(id);
                            }
                            self.mark_session_started();

                            (resp_text, lost)
                        } else {
                            if output_lower.contains("warning: conversation")
                                && output_lower.contains("not found")
                            {
                                self.reset_cli_session();
                                return TurnOutcome::SessionLost {
                                    error: "Previous session was not found on agent backend; continuing in a new conversation session.".to_string(),
                                    notice: trimmed.to_string(),
                                };
                            }
                            (trimmed.to_string(), false)
                        }
                    }
                    _ => {
                        // For Pinned or None, do not parse response text for conversation_id
                        (trimmed.to_string(), false)
                    }
                };

                let reply_clean = if final_reply.is_empty() {
                    "Agent returned an empty response.".to_string()
                } else {
                    final_reply
                };

                self.mark_session_started();

                if session_was_reset {
                    TurnOutcome::SessionLost {
                        error: "Previous conversation expired on backend; response was generated in a new session.".to_string(),
                        notice: reply_clean,
                    }
                } else {
                    TurnOutcome::Success { reply: reply_clean }
                }
            }
            Err(err) => {
                let mut clean_err = err.clone();
                for line in err.lines() {
                    let l = line.trim();
                    if l.starts_with('{')
                        && l.ends_with('}')
                        && let Ok(v) = serde_json::from_str::<serde_json::Value>(l)
                        && let Some(msg_val) = v.get("message").or_else(|| v.get("error"))
                        && let Some(s) = msg_val.as_str()
                    {
                        if let Ok(inner) = serde_json::from_str::<serde_json::Value>(s) {
                            if let Some(m) = inner
                                .get("error")
                                .and_then(|e| e.get("message"))
                                .and_then(|m| m.as_str())
                            {
                                clean_err = m.to_string();
                                break;
                            }
                        } else {
                            clean_err = s.to_string();
                            break;
                        }
                    }
                }

                let err_lower = clean_err.to_lowercase();
                let is_session_lost = err_lower.contains("session not found")
                    || err_lower.contains("conversation not found")
                    || err_lower.contains("invalid session")
                    || err_lower.contains("session expired")
                    || err_lower.contains("no conversation found");

                if is_session_lost {
                    self.reset_cli_session();
                    TurnOutcome::SessionLost {
                        error: clean_err.clone(),
                        notice: "Previous conversation session was not found or expired. Session has been reset for the next message.".to_string(),
                    }
                } else if !self.session_started {
                    self.reset_cli_session();
                    TurnOutcome::Error { error: clean_err }
                } else {
                    TurnOutcome::Error { error: clean_err }
                }
            }
        }
    }
}

#[note("session-queuedmessage")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueuedMessage {
    pub id: String,
    pub text: String,
    pub timestamp: String,
    #[serde(default)]
    pub has_image: bool,
    #[serde(default)]
    pub attachments: Vec<crate::chat_attachment::SelectionAttachment>,
    #[serde(default)]
    pub images: Vec<crate::chat_attachment::ImageAttachment>,
}

impl QueuedMessage {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            timestamp: "now".into(),
            has_image: false,
            attachments: Vec::new(),
            images: Vec::new(),
        }
    }

    pub fn with_image(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            timestamp: "now".into(),
            has_image: true,
            attachments: Vec::new(),
            images: Vec::new(),
        }
    }

    pub fn with_attachments(mut self, attachments: Vec<SelectionAttachment>) -> Self {
        self.attachments = attachments;
        self
    }

    pub fn with_images(mut self, images: Vec<crate::chat_attachment::ImageAttachment>) -> Self {
        if !images.is_empty() {
            self.has_image = true;
        }
        self.images = images;
        self
    }
}
