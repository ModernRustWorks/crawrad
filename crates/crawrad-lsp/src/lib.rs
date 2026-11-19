//! Language Server Protocol (LSP) client crate for the CrawRAD IDE.
//!
//! Provides JSON-RPC 2.0 communication over stdio with `rust-analyzer`, handling
//! document synchronization, diagnostics, code completion, hover tooltips, navigation
//! (definition, declaration, type, implementations, references), rename and formatting.
#![forbid(unsafe_code)]

pub mod asker;
pub mod client;
pub mod code_action;
pub mod code_lens;
pub mod complete;
pub mod definition;
pub mod edit;
pub mod encoding;
pub mod format;
pub mod highlight;
pub mod hover;
pub mod inlay;
pub mod locate;
pub mod protocol;
pub mod pull;
pub mod references;
pub mod rename;
pub mod resolve;
pub mod semantic;
pub(crate) mod settler;
pub mod signature;
pub mod symbols;
pub mod sync;
/// The framing moved to `crawrad-framing`: LSP and DAP share it.
pub use crawrad_framing as transport;
pub mod uri;

#[cfg(test)]
mod tests;

pub use client::{LspClient, LspError};
pub use code_lens::{CodeLens, CodeLensAnswer, CodeLensWorker, LensCommand, LensKind};
pub use complete::CompletionWorker;
pub use edit::{TextEdit, WorkspaceEdit, apply_text_edits, workspace_edit_from};
pub use highlight::{EditorAssist, HighlightWorker, Occurrence};
pub use inlay::{InlayAnswer, InlayHint, InlayWorker};
pub use locate::{LocatedServer, ServerMissing, ServerSource};
pub use protocol::{
    CompletionItem, CompletionItemKind, CompletionResponse, Diagnostic, DiagnosticSeverity,
    Documentation, Hover, Location, Mark, Position, PublishDiagnosticsParams, Range, line_messages,
    locations_from, marks,
};
pub use uri::{path_to_uri, uri_to_path};
