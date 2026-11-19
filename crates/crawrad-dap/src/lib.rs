//! The Debug Adapter Protocol client of CrawRAD (ADR 0048).
//!
//! The IDE does not debug anything itself: an external adapter (`lldb-dap`)
//! stops, steps and reads memory, and this crate is the client that talks to
//! it. It is **language-agnostic** on purpose — it knows the protocol, not
//! Rust. How a raw value becomes readable text is a [`ValuePresenter`]'s job,
//! one per language, living in its own crate (`crawrad-dap-rust` for Rust).
//!
//! No async runtime: a reader thread and channels, like `crawrad-lsp`.
#![forbid(unsafe_code)]

pub mod adapter;
pub mod base64;
pub mod client;
pub mod connection;
pub mod controller;
pub mod presenter;
pub mod protocol;
pub mod session;

pub use adapter::AdapterMissing;
pub use client::DapClient;
pub use connection::{DapConnection, DapError, PendingResponse};
pub use controller::{Command, DebugController, ShownVariable, Stop, Update, render_log_message};
pub use presenter::{AdapterText, FrameScoped, ValuePresenter, ValueSource};
pub use protocol::{Event, ProtocolMessage, Request, Response, Variable};
pub use session::{
    Breakpoint, DebugSession, LaunchConfig, SessionState, SourceBreakpoint, StackFrame,
};

#[cfg(test)]
#[path = "test/adapter.rs"]
mod adapter_tests;
#[cfg(test)]
#[path = "test/connection.rs"]
mod connection_tests;
#[cfg(test)]
#[path = "test/controller.rs"]
mod controller_tests;
#[cfg(test)]
#[path = "test/session.rs"]
mod session_tests;
#[cfg(test)]
#[path = "test/fake.rs"]
mod test_fake;
