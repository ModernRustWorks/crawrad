//! # Native Message Dialogs & Modal Components
//!
//! Provides toolkit-agnostic native dialogs (`ShowMessage`, `ShowWarning`,
//! `ShowError`, `ShowConfirm`, `MessageDlg`) matching the classic RAD simplicity and
//! maintaining the dark, polished theme of the host engine (Makepad / egui).
//!
//! Eliminates reliance on Win32 white `MessageBoxW` via `rfd`.

use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

pub use crawrad_component::pieces::ModalResult;
use crawrad_note::note;

#[note("dialogs-dialogkind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogKind {
    /// Informational notice with cyan info icon.
    Information,
    /// Warning alert with amber warning icon.
    Warning,
    /// Critical error with red/coral error icon.
    Error,
    /// Question / confirmation dialog with orange question icon.
    Confirmation,
    /// Custom message.
    Custom,
}

impl DialogKind {
    #[note("dialogs-dialogkind-default-title")]
    pub const fn default_title(&self) -> &'static str {
        match self {
            Self::Information => "Information",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Confirmation => "Confirm",
            Self::Custom => "Message",
        }
    }

    #[note("dialogs-dialogkind-icon-symbol")]
    pub const fn icon_symbol(&self) -> &'static str {
        match self {
            Self::Information => "ℹ",
            Self::Warning => "⚠",
            Self::Error => "✖",
            Self::Confirmation => "?",
            Self::Custom => "ℹ",
        }
    }

    #[note("dialogs-dialogkind-icon-color-rgb")]
    pub const fn icon_color_rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::Information => (121, 192, 255), // #79c0ff
            Self::Warning => (241, 163, 60),      // #f1a33c
            Self::Error => (255, 123, 114),       // #ff7b72
            Self::Confirmation => (222, 90, 43),  // #de5a2b (Rust Orange)
            Self::Custom => (205, 214, 244),      // #cdd6f4
        }
    }

    #[note("dialogs-dialogkind-from-name")]
    pub fn from_name(name: &str) -> Self {
        match name {
            "mtInformation" | "Information" => Self::Information,
            "mtWarning" | "Warning" => Self::Warning,
            "mtError" | "Error" => Self::Error,
            "mtConfirmation" | "Confirmation" => Self::Confirmation,
            "Custom" => Self::Custom,
            _ => Self::Information,
        }
    }

    #[note("dialogs-dialogkind-as-name")]
    pub const fn as_name(&self) -> &'static str {
        match self {
            Self::Information => "mtInformation",
            Self::Warning => "mtWarning",
            Self::Error => "mtError",
            Self::Confirmation => "mtConfirmation",
            Self::Custom => "Custom",
        }
    }
}

#[note("dialogs-dialogbuttons")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogButtons {
    /// Single OK button.
    Ok,
    /// OK and Cancel buttons.
    OkCancel,
    /// Yes and No buttons.
    YesNo,
    /// Yes, No, and Cancel buttons.
    YesNoCancel,
}

impl DialogButtons {
    #[note("dialogs-dialogbuttons-button-labels")]
    pub fn button_labels(&self) -> Vec<(&'static str, ModalResult)> {
        match self {
            Self::Ok => vec![("OK", ModalResult::Ok)],
            Self::OkCancel => vec![("OK", ModalResult::Ok), ("Cancel", ModalResult::Cancel)],
            Self::YesNo => vec![("Yes", ModalResult::Yes), ("No", ModalResult::No)],
            Self::YesNoCancel => vec![
                ("Yes", ModalResult::Yes),
                ("No", ModalResult::No),
                ("Cancel", ModalResult::Cancel),
            ],
        }
    }

    #[note("dialogs-dialogbuttons-default-result")]
    pub fn default_result(&self) -> ModalResult {
        match self {
            Self::Ok | Self::OkCancel => ModalResult::Ok,
            Self::YesNo | Self::YesNoCancel => ModalResult::Yes,
        }
    }

    #[note("dialogs-dialogbuttons-cancel-result")]
    pub fn cancel_result(&self) -> ModalResult {
        match self {
            Self::Ok => ModalResult::Ok,
            Self::OkCancel | Self::YesNoCancel => ModalResult::Cancel,
            Self::YesNo => ModalResult::No,
        }
    }

    #[note("dialogs-dialogbuttons-from-name")]
    pub fn from_name(name: &str) -> Self {
        match name {
            "mbOK" | "Ok" => Self::Ok,
            "mbOKCancel" | "OkCancel" => Self::OkCancel,
            "mbYesNo" | "YesNo" => Self::YesNo,
            "mbYesNoCancel" | "YesNoCancel" => Self::YesNoCancel,
            _ => Self::Ok,
        }
    }

    #[note("dialogs-dialogbuttons-as-name")]
    pub const fn as_name(&self) -> &'static str {
        match self {
            Self::Ok => "mbOK",
            Self::OkCancel => "mbOKCancel",
            Self::YesNo => "mbYesNo",
            Self::YesNoCancel => "mbYesNoCancel",
        }
    }
}

#[note("dialogs-pendinganswer")]
#[derive(Clone, Debug, Default)]
pub struct PendingAnswer(std::sync::Arc<std::sync::Mutex<Option<ModalResult>>>);

impl PendingAnswer {
    #[note("dialogs-pendinganswer-open")]
    pub fn open(&self, title: &str, message: &str, kind: DialogKind, buttons: DialogButtons) {
        let title = if title.is_empty() {
            kind.default_title()
        } else {
            title
        };
        let slot = self.0.clone();
        message_dlg(
            title,
            message,
            kind,
            buttons,
            Some(move |res| {
                if let Ok(mut lock) = slot.lock() {
                    *lock = Some(res);
                }
            }),
        );
    }

    #[note("dialogs-pendinganswer-take")]
    pub fn take(&self) -> Option<ModalResult> {
        self.0.lock().ok().and_then(|mut lock| lock.take())
    }
}

#[note("dialogs-dialogrequest")]
pub struct DialogRequest {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub kind: DialogKind,
    pub buttons: DialogButtons,
    pub callback: Option<Box<dyn FnOnce(ModalResult) + Send + 'static>>,
}

#[note("dialogs-dialogsnapshot")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogSnapshot {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub kind: DialogKind,
    pub buttons: DialogButtons,
}

#[note("dialogs-dialogmanager")]
#[derive(Default)]
pub struct DialogManager {
    next_id: u64,
    active: Option<DialogRequest>,
    queue: VecDeque<DialogRequest>,
}

impl DialogManager {
    #[note("dialogs-dialogmanager-post")]
    pub fn post(&mut self, mut req: DialogRequest) -> u64 {
        self.next_id += 1;
        req.id = self.next_id;
        if self.active.is_none() {
            self.active = Some(req);
        } else {
            self.queue.push_back(req);
        }
        self.next_id
    }

    #[note("dialogs-dialogmanager-active")]
    pub fn active(&self) -> Option<&DialogRequest> {
        self.active.as_ref()
    }

    #[note("dialogs-dialogmanager-snapshot")]
    pub fn snapshot(&self) -> Option<DialogSnapshot> {
        self.active.as_ref().map(|r| DialogSnapshot {
            id: r.id,
            title: r.title.clone(),
            message: r.message.clone(),
            kind: r.kind,
            buttons: r.buttons.clone(),
        })
    }

    #[note("dialogs-dialogmanager-resolve")]
    pub fn resolve(&mut self, result: ModalResult) -> Option<ModalResult> {
        if let Some(mut req) = self.active.take() {
            if let Some(cb) = req.callback.take() {
                cb(result);
            }
            self.active = self.queue.pop_front();
            Some(result)
        } else {
            None
        }
    }

    #[note("dialogs-dialogmanager-is-open")]
    pub fn is_open(&self) -> bool {
        self.active.is_some()
    }

    #[note("dialogs-dialogmanager-clear")]
    pub fn clear(&mut self) {
        self.active = None;
        self.queue.clear();
    }
}

static MANAGER: OnceLock<Mutex<DialogManager>> = OnceLock::new();

#[note("dialogs-dialog-manager")]
pub fn dialog_manager() -> &'static Mutex<DialogManager> {
    MANAGER.get_or_init(|| Mutex::new(DialogManager::default()))
}

#[note("dialogs-active-dialog")]
pub fn active_dialog() -> Option<DialogSnapshot> {
    dialog_manager().lock().unwrap().snapshot()
}

#[note("dialogs-is-dialog-open")]
pub fn is_dialog_open() -> bool {
    dialog_manager().lock().unwrap().is_open()
}

#[note("dialogs-resolve-dialog")]
pub fn resolve_dialog(result: ModalResult) -> Option<ModalResult> {
    dialog_manager().lock().unwrap().resolve(result)
}

#[note("dialogs-show-message")]
pub fn show_message(message: impl Into<String>) {
    let msg = message.into();
    let req = DialogRequest {
        id: 0,
        title: DialogKind::Information.default_title().to_string(),
        message: msg,
        kind: DialogKind::Information,
        buttons: DialogButtons::Ok,
        callback: None,
    };
    dialog_manager().lock().unwrap().post(req);
}

#[note("dialogs-show-warning")]
pub fn show_warning(message: impl Into<String>) {
    let msg = message.into();
    let req = DialogRequest {
        id: 0,
        title: DialogKind::Warning.default_title().to_string(),
        message: msg,
        kind: DialogKind::Warning,
        buttons: DialogButtons::Ok,
        callback: None,
    };
    dialog_manager().lock().unwrap().post(req);
}

#[note("dialogs-show-error")]
pub fn show_error(message: impl Into<String>) {
    let msg = message.into();
    let req = DialogRequest {
        id: 0,
        title: DialogKind::Error.default_title().to_string(),
        message: msg,
        kind: DialogKind::Error,
        buttons: DialogButtons::Ok,
        callback: None,
    };
    dialog_manager().lock().unwrap().post(req);
}

#[note("dialogs-show-confirm")]
pub fn show_confirm(
    message: impl Into<String>,
    callback: impl FnOnce(ModalResult) + Send + 'static,
) {
    let msg = message.into();
    let req = DialogRequest {
        id: 0,
        title: DialogKind::Confirmation.default_title().to_string(),
        message: msg,
        kind: DialogKind::Confirmation,
        buttons: DialogButtons::YesNo,
        callback: Some(Box::new(callback)),
    };
    dialog_manager().lock().unwrap().post(req);
}

#[note("dialogs-show-confirm-sync")]
pub fn show_confirm_sync(message: impl Into<String>) -> ModalResult {
    let (tx, rx) = std::sync::mpsc::channel();
    show_confirm(message, move |res| {
        let _ = tx.send(res);
    });
    rx.recv().unwrap_or(ModalResult::Cancel)
}

#[note("dialogs-message-dlg")]
pub fn message_dlg(
    title: impl Into<String>,
    message: impl Into<String>,
    kind: DialogKind,
    buttons: DialogButtons,
    callback: Option<impl FnOnce(ModalResult) + Send + 'static>,
) {
    let req = DialogRequest {
        id: 0,
        title: title.into(),
        message: message.into(),
        kind,
        buttons,
        callback: callback.map(|cb| Box::new(cb) as Box<dyn FnOnce(ModalResult) + Send + 'static>),
    };
    dialog_manager().lock().unwrap().post(req);
}
