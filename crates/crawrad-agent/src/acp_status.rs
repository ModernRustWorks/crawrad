//! The agent card's live status line: while the agent works, one line
//! follows every ACP `session/update`; when the turn ends it stops and dims.
//!
//! The formatting takes the elapsed time as an argument, so it is pure and
//! testable; only [`StatusTracker::current`] and [`StatusWatch`] read the clock.

use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::client::AcpStreamEvent;
use crate::markdown::ToolCallStatus;
use crawrad_note::note;

#[note("acp-status-thought-tail-chars")]
const THOUGHT_TAIL_CHARS: usize = 70;

#[note("acp-status-statusline")]
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StatusLine {
    pub line: String,
    pub sub: String,
    pub done: bool,
}

#[note("acp-status-step")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    Started,
    Thought,
    /// A tool is running (index into `tools`).
    Running(usize),
    /// A tool completed or failed (index into `tools`).
    ToolDone(usize),
    Finished,
}

#[note("acp-status-statustracker")]
pub struct StatusTracker {
    agent: String,
    via_cli: bool,
    started: Instant,
    /// The time the turn took, fixed by [`Self::finish`].
    took: Option<Duration>,
    thought: String,
    tools: Vec<(String, String, ToolCallStatus)>,
    step: Step,
}

impl StatusTracker {
    #[note("acp-status-statustracker-new")]
    pub fn new(agent: &str, via_cli: bool) -> Self {
        Self {
            agent: agent.to_string(),
            via_cli,
            started: Instant::now(),
            took: None,
            thought: String::new(),
            tools: Vec::new(),
            step: Step::Started,
        }
    }

    #[note("acp-status-statustracker-on-event")]
    pub fn on_event(&mut self, event: &AcpStreamEvent) -> bool {
        let before = self.at(Duration::ZERO);
        match event {
            AcpStreamEvent::ThoughtDelta(delta) => {
                self.thought.push_str(delta);
                self.step = Step::Thought;
            }
            AcpStreamEvent::ToolCall { id, name, .. } => {
                let index = self.tool_index(id, name, false);
                self.tools[index].2 = ToolCallStatus::Running;
                self.step = Step::Running(index);
            }
            AcpStreamEvent::ToolCallUpdate {
                id, name, status, ..
            } => {
                let index = self.tool_index(id, name, true);
                self.tools[index].2 = *status;
                self.step = match status {
                    ToolCallStatus::Pending | ToolCallStatus::Running => Step::Running(index),
                    ToolCallStatus::Success | ToolCallStatus::Failed => Step::ToolDone(index),
                };
            }
            _ => return false,
        }
        self.at(Duration::ZERO) != before
    }

    #[note("acp-status-statustracker-tool-index")]
    fn tool_index(&mut self, id: &str, name: &str, by_name: bool) -> usize {
        let by_id = self
            .tools
            .iter()
            .position(|(tid, _, _)| !id.is_empty() && tid == id);
        let found = by_id.or_else(|| {
            let by_title = |t: &(String, String, ToolCallStatus)| t.1 == name;
            by_name
                .then(|| self.tools.iter().position(by_title))
                .flatten()
        });
        match found {
            Some(index) => {
                if !name.is_empty() {
                    self.tools[index].1 = name.to_string();
                }
                index
            }
            None => {
                let status = ToolCallStatus::Pending;
                self.tools.push((id.to_string(), name.to_string(), status));
                self.tools.len() - 1
            }
        }
    }

    #[note("acp-status-statustracker-finish")]
    pub fn finish(&mut self) -> StatusLine {
        self.took = Some(self.started.elapsed());
        self.step = Step::Finished;
        self.current()
    }

    #[note("acp-status-statustracker-current")]
    pub fn current(&self) -> StatusLine {
        self.at(self.took.unwrap_or_else(|| self.started.elapsed()))
    }

    #[note("acp-status-statustracker-at")]
    pub fn at(&self, elapsed: Duration) -> StatusLine {
        let secs = elapsed.as_secs();
        let tool = |index: usize| self.tools.get(index).map(|t| t.1.as_str()).unwrap_or("");
        let (line, sub) = match self.step {
            Step::Started if self.via_cli => (
                format!(
                    "Sending the prompt to {} through the CLI\u{2026}",
                    self.agent
                ),
                "waiting for the reply".to_string(),
            ),
            Step::Started => (
                format!(
                    "ACP session started. Sending the prompt to {}\u{2026}",
                    self.agent
                ),
                "waiting for the first update".to_string(),
            ),
            Step::Thought => (
                format!("Thinking: {}\u{2026}", thought_tail(&self.thought)),
                format!("thought \u{b7} {secs} s"),
            ),
            Step::Running(index) => (
                format!("Running tool: {}", tool(index)),
                format!("tool_call \u{b7} in_progress \u{b7} {secs} s"),
            ),
            Step::ToolDone(index) => {
                let failed = self.tools.get(index).map(|t| t.2) == Some(ToolCallStatus::Failed);
                let (word, state) = if failed {
                    ("failed", "failed")
                } else {
                    ("ok", "completed")
                };
                (
                    format!("Tool done: {} ({word})", tool(index)),
                    format!("tool_call \u{b7} {state}"),
                )
            }
            Step::Finished => (
                format!("Done in {secs} s{}", tool_count(self.tools.len())),
                String::new(),
            ),
        };
        StatusLine {
            line,
            sub,
            done: self.step == Step::Finished,
        }
    }
}

#[note("acp-status-tool-count")]
fn tool_count(count: usize) -> String {
    match count {
        0 => String::new(),
        1 => " \u{b7} 1 tool".to_string(),
        n => format!(" \u{b7} {n} tools"),
    }
}

#[note("acp-status-thought-tail")]
fn thought_tail(thought: &str) -> String {
    let one_line = thought.split_whitespace().collect::<Vec<_>>().join(" ");
    let count = one_line.chars().count();
    one_line
        .chars()
        .skip(count.saturating_sub(THOUGHT_TAIL_CHARS))
        .collect()
}

#[note("acp-status-with-elapsed")]
pub fn with_elapsed(status: &StatusLine, elapsed: Duration) -> StatusLine {
    let mut out = status.clone();
    if status.done || !status.sub.ends_with(" s") {
        return out;
    }
    if let Some((head, _)) = status.sub.rsplit_once(" \u{b7} ") {
        out.sub = format!("{head} \u{b7} {} s", elapsed.as_secs());
    }
    out
}

#[note("acp-status-statusfeed")]
pub struct StatusFeed {
    tracker: StatusTracker,
    tx: Sender<StatusLine>,
}

impl StatusFeed {
    #[note("acp-status-statusfeed-start")]
    pub fn start(agent: &str, via_cli: bool, tx: Sender<StatusLine>) -> Self {
        let feed = Self {
            tracker: StatusTracker::new(agent, via_cli),
            tx,
        };
        feed.send();
        feed
    }

    fn send(&self) {
        let _ = self.tx.send(self.tracker.current());
        crawrad_os::tasks::wake();
    }

    #[note("acp-status-statusfeed-on-event")]
    pub fn on_event(&mut self, event: &AcpStreamEvent) {
        if self.tracker.on_event(event) {
            self.send();
        }
    }

    #[note("acp-status-statusfeed-fall-back-to-cli")]
    pub fn fall_back_to_cli(&mut self) {
        let started = self.tracker.started;
        self.tracker = StatusTracker::new(&self.tracker.agent, true);
        self.tracker.started = started;
        self.send();
    }

    #[note("acp-status-statusfeed-finish")]
    pub fn finish(&mut self) -> StatusLine {
        let last = self.tracker.finish();
        let _ = self.tx.send(last.clone());
        crawrad_os::tasks::wake();
        last
    }
}

#[note("acp-status-statuswatch")]
pub struct StatusWatch {
    rx: Receiver<StatusLine>,
    latest: StatusLine,
    started: Instant,
}

impl StatusWatch {
    #[note("acp-status-statuswatch-channel")]
    pub fn channel(agent: &str) -> (Sender<StatusLine>, Self) {
        let (tx, rx) = channel();
        let watch = Self {
            rx,
            latest: StatusTracker::new(agent, false).at(Duration::ZERO),
            started: Instant::now(),
        };
        (tx, watch)
    }

    #[note("acp-status-statuswatch-poll")]
    pub fn poll(&mut self) -> StatusLine {
        while let Ok(line) = self.rx.try_recv() {
            self.latest = line;
        }
        self.current()
    }

    #[note("acp-status-statuswatch-current")]
    pub fn current(&self) -> StatusLine {
        with_elapsed(&self.latest, self.started.elapsed())
    }
}

#[cfg(test)]
#[path = "test/acp_status/test.rs"]
mod tests;
