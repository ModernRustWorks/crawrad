//! The chat's past conversations: one JSON file per conversation, with
//! its messages, the agent that answered, its ACP session and when it was last
//! used. The history button lists them newest first; opening one restores its
//! turns and, when the agent offers `session/load`, the same ACP session.
#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::session::{ChatMessage, MessageSender};
use crawrad_note::note;

#[note("chat-history-conversation")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    /// The agent's name as the chat shows it (`Claude`, `Gemini`).
    pub agent: String,
    /// Which agent configuration answered (`LiveAgent::key_of`): a session
    /// is only resumed by that very agent.
    pub agent_key: String,
    /// The ACP session id, when the conversation ran over ACP.
    #[serde(default)]
    pub acp_session: Option<String>,
    /// Whether the agent declared `loadSession` (it can resume this session).
    #[serde(default)]
    pub can_resume: bool,
    /// When it was last used, local `"YYYY-MM-DD HH:MM"`.
    pub updated: String,
    pub messages: Vec<ChatMessage>,
}

impl Conversation {
    #[note("chat-history-conversation-new-id")]
    pub fn new_id() -> String {
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let tail = uuid::Uuid::new_v4().simple().to_string();
        format!("c{millis}-{}", &tail[..8])
    }

    #[note("chat-history-conversation-title")]
    pub fn title(&self) -> String {
        let first = self
            .messages
            .iter()
            .find(|m| m.sender == MessageSender::User)
            .map(|m| m.text.trim().lines().next().unwrap_or_default().trim())
            .unwrap_or_default();
        match first.chars().count() {
            0 => "(empty conversation)".to_string(),
            n if n > 80 => format!("{}…", first.chars().take(79).collect::<String>()),
            _ => first.to_string(),
        }
    }

    #[note("chat-history-conversation-turns")]
    pub fn turns(&self) -> usize {
        self.messages
            .iter()
            .filter(|m| m.sender == MessageSender::User)
            .count()
    }

    #[note("chat-history-conversation-summary")]
    pub fn summary(&self, today: &str) -> Value {
        json!({
            "id": self.id,
            "title": self.title(),
            "agent": self.agent,
            "turns": self.turns(),
            "group": group_of(&self.updated, today),
            "when": when_of(&self.updated, today),
            "resumes": self.can_resume && self.acp_session.is_some(),
        })
    }
}

#[note("chat-history-group-of")]
pub fn group_of(stamp: &str, today: &str) -> &'static str {
    match (day_number(stamp), day_number(today)) {
        (Some(d), Some(t)) if d == t => "Today",
        (Some(d), Some(t)) if d + 1 == t => "Yesterday",
        _ => "Earlier",
    }
}

#[note("chat-history-when-of")]
pub fn when_of(stamp: &str, today: &str) -> String {
    if group_of(stamp, today) != "Earlier" {
        return stamp.get(11..16).unwrap_or_default().to_string();
    }
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let month = stamp.get(5..7).and_then(|m| m.parse::<usize>().ok());
    let day = stamp.get(8..10).and_then(|d| d.parse::<u32>().ok());
    match (month, day) {
        (Some(m @ 1..=12), Some(d)) => format!("{} {d}", MONTHS[m - 1]),
        _ => stamp.to_string(),
    }
}

fn day_number(stamp: &str) -> Option<i64> {
    let y = stamp.get(0..4)?.parse().ok()?;
    let m = stamp.get(5..7)?.parse().ok()?;
    let d = stamp.get(8..10)?.parse().ok()?;
    Some(crawrad_os::clock::days_from_civil(y, m, d))
}

#[note("chat-history-historystore")]
#[derive(Debug, Clone)]
pub struct HistoryStore {
    dir: PathBuf,
}

impl HistoryStore {
    #[note("chat-history-historystore-at")]
    pub fn at(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    #[note("chat-history-historystore-user-default")]
    pub fn user_default() -> Self {
        let base = std::env::var_os("APPDATA")
            .map(|a| PathBuf::from(a).join("CrawRAD"))
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".crawrad")))
            .unwrap_or_else(|| PathBuf::from(".crawrad"));
        Self::at(base.join("chat-history"))
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn file(&self, id: &str) -> Option<PathBuf> {
        let safe = !id.is_empty()
            && id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        safe.then(|| self.dir.join(format!("{id}.json")))
    }

    #[note("chat-history-historystore-save")]
    pub fn save(&self, conversation: &Conversation) -> Result<(), String> {
        if conversation.turns() == 0 {
            return Ok(());
        }
        let file = self
            .file(&conversation.id)
            .ok_or_else(|| format!("invalid conversation id '{}'", conversation.id))?;
        std::fs::create_dir_all(&self.dir)
            .map_err(|e| format!("could not create '{}': {e}", self.dir.display()))?;
        let text = serde_json::to_string_pretty(conversation).map_err(|e| e.to_string())?;
        let temp = file.with_extension("json.tmp");
        std::fs::write(&temp, text)
            .map_err(|e| format!("could not write '{}': {e}", temp.display()))?;
        std::fs::rename(&temp, &file)
            .map_err(|e| format!("could not write '{}': {e}", file.display()))
    }

    #[note("chat-history-historystore-load")]
    pub fn load(&self, id: &str) -> Option<Conversation> {
        let text = std::fs::read_to_string(self.file(id)?).ok()?;
        serde_json::from_str(&text).ok()
    }

    #[note("chat-history-historystore-list")]
    pub fn list(&self) -> Vec<Conversation> {
        let mut all: Vec<Conversation> = std::fs::read_dir(&self.dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
            .filter_map(|e| std::fs::read_to_string(e.path()).ok())
            .filter_map(|t| serde_json::from_str(&t).ok())
            .collect();
        all.sort_by(|a, b| b.updated.cmp(&a.updated).then_with(|| b.id.cmp(&a.id)));
        all
    }

    #[note("chat-history-historystore-summaries")]
    pub fn summaries(&self, today: &str, query: &str) -> Vec<Value> {
        let query = query.trim().to_lowercase();
        self.list()
            .iter()
            .filter(|c| {
                query.is_empty()
                    || c.title().to_lowercase().contains(&query)
                    || c.agent.to_lowercase().contains(&query)
            })
            .map(|c| c.summary(today))
            .collect()
    }
}

#[note("chat-history-record")]
pub fn record(
    store: &HistoryStore,
    open: &mut Option<String>,
    agent: &str,
    live: &crate::acp_live::LiveAgent,
    messages: &[ChatMessage],
) -> Result<(), String> {
    let id = open.get_or_insert_with(Conversation::new_id).clone();
    let last_session = messages.iter().rev().find_map(|m| m.acp_session.clone());
    store.save(&Conversation {
        id,
        agent: agent.to_string(),
        agent_key: live.agent_key().to_string(),
        acp_session: live.session_id().map(str::to_string).or(last_session),
        can_resume: live.can_resume(),
        updated: crawrad_os::clock::local_stamp(),
        messages: messages.to_vec(),
    })
}

#[note("chat-history-reopen")]
pub fn reopen(
    store: &HistoryStore,
    id: &str,
    live: &mut crate::acp_live::LiveAgent,
) -> Option<Vec<ChatMessage>> {
    let conversation = store.load(id)?;
    let same_agent = conversation.agent_key == live.agent_key();
    let resumes = conversation.can_resume && same_agent;
    live.resume_next(conversation.acp_session.filter(|_| resumes));
    Some(conversation.messages)
}

#[note("chat-history-mcp-json")]
pub fn mcp_json(store: &HistoryStore, open: Option<&str>, query: &str) -> Value {
    let stamp = crawrad_os::clock::local_stamp();
    let today = stamp.get(..10).unwrap_or_default();
    json!({ "conversation": open, "history": store.summaries(today, query) })
}

#[note("chat-history-mcp-user-json")]
pub fn mcp_user_json(open: Option<&str>, args: &Value) -> Value {
    let query = args["query"].as_str().unwrap_or("");
    mcp_json(&HistoryStore::user_default(), open, query)
}

#[note("chat-history-row")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: String,
    /// Today / Yesterday / Earlier, only on the first row of its group.
    pub group: Option<String>,
    pub title: String,
    /// `Claude · 4 turns`.
    pub meta: String,
    pub when: String,
    /// `resumes` (the agent goes on in the same ACP session) or `read only`.
    pub resume: &'static str,
    /// The conversation open in the chat now (the orange bar).
    pub open: bool,
}

#[note("chat-history-rows")]
pub fn rows(summaries: &[Value], open: Option<&str>, max: usize) -> Vec<Row> {
    let mut last_group = None;
    let text = |v: &Value, k: &str| v[k].as_str().unwrap_or_default().to_string();
    summaries
        .iter()
        .take(max)
        .map(|s| {
            let group = text(s, "group");
            let turns = s["turns"].as_u64().unwrap_or(0);
            let unit = if turns == 1 { "turn" } else { "turns" };
            let first = last_group.as_ref() != Some(&group);
            last_group = Some(group.clone());
            Row {
                id: text(s, "id"),
                group: first.then_some(group),
                title: text(s, "title"),
                meta: format!("{} · {turns} {unit}", text(s, "agent")),
                when: text(s, "when"),
                resume: if s["resumes"].as_bool() == Some(true) {
                    "resumes"
                } else {
                    "read only"
                },
                open: open == s["id"].as_str(),
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "test/chat_history/test.rs"]
mod tests;
