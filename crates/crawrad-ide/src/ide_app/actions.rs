//! ACTION DISPATCH of the shell — delegated by UI family.
//!
//! # Why a folder and modules per area
//!
//! `handle_actions` had 1,707 lines and was the point of greatest friction
//! between concurrent IDE features. Each module handles a self-contained area
//! (input, menu, editor, designer, panels, chat, terminal, dialogs).

#[path = "actions/chat.rs"]
pub(crate) mod chat;
#[path = "actions/designer.rs"]
pub(crate) mod designer;
#[path = "actions/dialogs.rs"]
pub(crate) mod dialogs;
#[path = "actions/editor.rs"]
pub(crate) mod editor;
#[path = "actions/input.rs"]
pub(crate) mod input;
#[path = "actions/menu.rs"]
pub(crate) mod menu;
#[path = "actions/panels.rs"]
pub(crate) mod panels;
#[path = "actions/terminal.rs"]
pub(crate) mod terminal;
