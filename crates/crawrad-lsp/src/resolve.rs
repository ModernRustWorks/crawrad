//! **The completion item's documentation** off the window's thread.
//!
//! The list comes without documentation: the client announces
//! `completionItem.resolveSupport` for `documentation`, so the server leaves it
//! out and fills it in `completionItem/resolve` for the one item selected (LSP
//! 3.17). Measured on rust-analyzer (2026-10-04, `s.` on a `String`): 123
//! items, 126 KB with the documentation inline, 63 KB without; resolving `len`
//! with only its `label` and `data` answers at once with its markdown.
//!
//! Moving fast over the list does not pile requests up: the engine
//! ([`crate::asker`]) asks only the newest, and an answer for any other ticket
//! is dropped.

use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;

use crate::client::LspClient;
use crawrad_note::note;

#[note("resolve-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocAnswer {
    pub ticket: u64,
    /// The documentation made plain, or `None` when the item has none.
    pub doc: Option<String>,
}

#[note("resolve-docworker")]
pub struct DocWorker(crate::asker::Asker<DocAnswer>);

impl Default for DocWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl DocWorker {
    pub fn new() -> Self {
        Self(crate::asker::Asker::new("lsp-resolve"))
    }

    #[note("resolve-docworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("resolve-docworker-request")]
    pub fn request(&self, client: Arc<LspClient>, ticket: u64, label: &str, data: Value) -> bool {
        let item = serde_json::json!({ "label": label, "data": data });
        self.0.ask(ticket, move || {
            let answer = client.send_request("completionItem/resolve", item, TIMEOUT);
            let doc = answer
                .ok()
                .and_then(|v| v.get("documentation").and_then(documentation));
            DocAnswer { ticket, doc }
        })
    }

    #[note("resolve-docworker-take")]
    pub fn take(&self, ticket: u64) -> Option<DocAnswer> {
        self.0.take(ticket)
    }
}

#[note("resolve-documentation")]
pub fn documentation(doc: &Value) -> Option<String> {
    let text = match doc {
        Value::String(s) => s.clone(),
        other => other.get("value")?.as_str()?.to_string(),
    };
    let plain = crate::hover::plain(&text);
    (!plain.trim().is_empty()).then_some(plain)
}

#[cfg(test)]
#[path = "test/resolve/test.rs"]
mod tests;
