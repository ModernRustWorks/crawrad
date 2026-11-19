//! The AI chat state, session, queue, and audio recording (ADR 0043, PR 7).

use makepad_widgets::Timer;
use std::sync::mpsc::{Receiver, Sender};

use crate::ide_app::{AiPermQueue, AiPermRx, AiSessionPerms, AiTurnResult, AudioSessionInstance};
use crawrad_agent::ChatSession;
use crawrad_note::note;

#[note("ide-app-chat-state-chatstate")]
#[derive(Default)]
pub struct ChatState {
    pub session: ChatSession,
    /// The open conversation's id in the history, once its first turn ended.
    pub conversation: Option<String>,
    /// The id under each drawn row of the history list.
    pub history_rows: Vec<String>,
    pub queue: Vec<crawrad_agent::QueuedMessage>,
    /// Collapsed is the exception: the queue opens expanded.
    pub queue_collapsed: bool,
    pub busy: bool,
    pub acp_status_msg: Option<String>,
    pub opt_ai_catalogue: bool,
    pub opt_ai_advanced: bool,
    pub acp_registry_query: String,
    pub acp_registry_feedback: Option<String>,
    pub audio_gravando: bool,
    pub audio_prompt_anterior: String,
    pub audio_sessao: Option<AudioSessionInstance>,
    /// Clock ticking elapsed seconds while recording audio.
    pub recording_clock: Timer,
    /// Clock ticking elapsed seconds while the AI agent is busy.
    pub busy_clock: Timer,
    pub resposta_rx: Option<Receiver<AiTurnResult>>,
    pub status: Option<crawrad_agent::acp_status::StatusWatch>,
    /// The `Form` / `Sel` switches, stored as OFF so the default is ON.
    pub form_off: bool,
    pub selection_off: bool,
    /// Each visible reply card's markdown, for its Copy button.
    pub copy_texts: Vec<String>,
    pub cancel_tx: Option<Sender<()>>,
    pub options_test_rx: Option<Receiver<Result<String, String>>>,
    pub acp_install_rx: Option<Receiver<Result<String, String>>>,
    pub probe_rx: Option<Receiver<Result<String, String>>>,
    pub scroll_to_bottom_frames: u8,
    pub permission_queue: AiPermQueue,
    pub session_permissions: AiSessionPerms,
    pub live: crawrad_agent::acp_live::LiveAgent,
    pub permission_rx: Option<AiPermRx>,
    pub writes: crawrad_agent::AgentWrites,
    pub edit_asks: crawrad_agent::AgentEditAsks,
    pub open_menu: Option<String>,
    pub slash_selected: usize,
    /// Selections captured for the next message, drawn as chips.
    pub attachments: Vec<crawrad_agent::SelectionAttachment>,
    pub images: Vec<crawrad_agent::ImageAttachment>,
}
