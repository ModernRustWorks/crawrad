//! **Items of the workspace by name** (`workspace/symbol`, LSP 3.17), for
//! the note dialog's *Link item…*.
//!
//! rust-analyzer lists only types unless the query ends in `#`
//! (measured on 1.95: `fetch` finds the trait `Fetcher`, `fetch#` also the
//! method `fetch`), so the query is sent with it. A method's or a variant's
//! `containerName` is its type (`RegistryClient`, `RegistryError`); an item
//! of a module has none.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::client::LspClient;
use crawrad_note::note;

#[note("symbols-timeout")]
pub const TIMEOUT: Duration = Duration::from_secs(5);

#[note("symbols-symbol")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub container: Option<String>,
    /// LSP `SymbolKind` (12 is a function, 6 a method, 22 a variant).
    pub kind: u32,
    pub file: PathBuf,
    pub line: usize,
}

#[note("symbols-symbolanswer")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolAnswer {
    pub ticket: u64,
    pub outcome: Result<Vec<Symbol>, String>,
}

#[note("symbols-symbolworker")]
pub struct SymbolWorker(crate::asker::Asker<SymbolAnswer>);

impl Default for SymbolWorker {
    fn default() -> Self {
        Self(crate::asker::Asker::new("lsp-symbols"))
    }
}

impl SymbolWorker {
    #[note("symbols-symbolworker-set-waker")]
    pub fn set_waker(&self, wake: impl Fn() + Send + 'static) {
        self.0.set_waker(wake);
    }

    #[note("symbols-symbolworker-request")]
    pub fn request(&self, client: Arc<LspClient>, ticket: u64, query: &str) -> bool {
        let params = serde_json::json!({ "query": format!("{query}#") });
        self.0.ask(ticket, move || {
            let mut request = |left| client.send_request("workspace/symbol", params.clone(), left);
            let outcome =
                crate::definition::retrying(&mut request, TIMEOUT, crate::definition::RETRY)
                    .map(parse)
                    .map_err(|e| e.to_string());
            SymbolAnswer { ticket, outcome }
        })
    }

    #[note("symbols-symbolworker-take")]
    pub fn take(&self, ticket: u64) -> Option<SymbolAnswer> {
        self.0.take(ticket)
    }
}

#[note("symbols-parse")]
pub fn parse(value: serde_json::Value) -> Vec<Symbol> {
    let items = value.as_array().cloned().unwrap_or_default();
    items
        .iter()
        .filter_map(|s| {
            let location = s.get("location")?;
            let file = crate::uri_to_path(location.get("uri")?.as_str()?)?;
            let line = location
                .pointer("/range/start/line")
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(0);
            Some(Symbol {
                name: s.get("name")?.as_str()?.to_string(),
                container: s
                    .get("containerName")
                    .and_then(serde_json::Value::as_str)
                    .filter(|c| !c.is_empty())
                    .map(str::to_string),
                kind: s
                    .get("kind")
                    .and_then(serde_json::Value::as_u64)
                    .unwrap_or(0) as u32,
                file,
                line: line as usize,
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "test/symbols/test.rs"]
mod tests;
