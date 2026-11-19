//! **Pull diagnostics** (LSP 3.17 `textDocument/diagnostic`).
//!
//! Measured on rust-analyzer 0.3.3065 (2026-10-26): `publishDiagnostics`
//! carries only what `cargo check` found (source `rustc`). The server's own
//! diagnostics — among them `inactive_code` over an item a `#[cfg]` turns
//! off, a Hint tagged Unnecessary — come only when the client asks for
//! them, file by file, with `textDocument/diagnostic`; the server answers
//! even a client that does not declare the capability, and keeps pushing
//! `rustc`'s.
//!
//! So each open, edit and save asks, without waiting: the answer arrives on
//! the reader's thread. A file's list is what was pushed plus what was
//! pulled, and goes out through the same channel the pushed ones always did,
//! so the editor draws both. `workspace/diagnostic/refresh`, and the end of
//! the server's work in progress, ask again for every open file.

use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::client::{LspError, ResponseCallback, Waker, file_key};
use crate::protocol::PublishDiagnosticsParams;
use crawrad_note::note;

#[note("pull-state")]
#[derive(Debug, Default)]
pub struct State {
    pushed: HashMap<String, Vec<Value>>,
    /// The newest answer and the document version it was asked for.
    pulled: HashMap<String, (i32, Vec<Value>)>,
    open: HashMap<String, (String, i32)>,
}

impl State {
    #[note("pull-state-asking")]
    pub fn asking(&mut self, key: &str, uri: &str, version: i32) {
        self.open
            .insert(key.to_string(), (uri.to_string(), version));
    }

    #[note("pull-state-pulled")]
    pub fn pulled(&mut self, key: &str, version: i32, items: Vec<Value>) -> bool {
        match self.pulled.get(key) {
            Some((kept, _)) if *kept > version => false,
            _ => {
                self.pulled.insert(key.to_string(), (version, items));
                true
            }
        }
    }

    #[note("pull-state-pushed")]
    pub fn pushed(&mut self, key: &str, items: Vec<Value>) {
        self.pushed.insert(key.to_string(), items);
    }

    #[note("pull-state-merged")]
    pub fn merged(&self, key: &str) -> Vec<Value> {
        let pushed = self.pushed.get(key).into_iter().flatten();
        let pulled = self.pulled.get(key).into_iter().flat_map(|(_, i)| i);
        pushed.chain(pulled).cloned().collect()
    }

    #[note("pull-state-closed")]
    pub fn closed(&mut self, key: &str) {
        self.open.remove(key);
        self.pulled.remove(key);
    }

    #[note("pull-state-open")]
    pub fn open(&self) -> Vec<(String, String, i32)> {
        let each = self.open.iter();
        each.map(|(k, (u, v))| (k.clone(), u.clone(), *v)).collect()
    }
}

#[note("pull-full-items")]
pub fn full_items(answer: &Value) -> Option<Vec<Value>> {
    if answer.get("kind").and_then(Value::as_str) != Some("full") {
        return None;
    }
    answer.get("items").and_then(Value::as_array).cloned()
}

#[note("pull-pull")]
pub struct Pull {
    /// Off until `initialize` says the server has a `diagnosticProvider`.
    pub enabled: AtomicBool,
    pub(crate) next_id: Arc<AtomicU64>,
    pub(crate) writer: Sender<String>,
    pub(crate) pending: Arc<Mutex<HashMap<u64, ResponseCallback>>>,
    /// The merged lists, as the code actions and the Explorer read them.
    pub(crate) published: Arc<Mutex<HashMap<String, Vec<Value>>>>,
    pub(crate) diagnostics: Sender<PublishDiagnosticsParams>,
    pub(crate) waker: Waker,
    pub(crate) state: Mutex<State>,
}

impl Pull {
    #[note("pull-pull-ask")]
    pub fn ask(self: &Arc<Self>, file: &Path, version: i32) {
        let (key, uri) = (file_key(file), crate::uri::path_to_uri(file));
        self.ask_key(key, uri, version);
    }

    #[note("pull-pull-ask-all")]
    pub fn ask_all(self: &Arc<Self>) {
        let open = self.state.lock().map(|s| s.open()).unwrap_or_default();
        for (key, uri, version) in open {
            self.ask_key(key, uri, version);
        }
    }

    #[note("pull-pull-forget")]
    pub fn forget(&self, file: &Path) {
        if let Ok(mut s) = self.state.lock() {
            s.closed(&file_key(file));
        }
    }

    fn ask_key(self: &Arc<Self>, key: String, uri: String, version: i32) {
        if !self.enabled.load(Ordering::SeqCst) {
            return;
        }
        if let Ok(mut s) = self.state.lock() {
            s.asking(&key, &uri, version);
        }
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let request = serde_json::json!({ "jsonrpc": "2.0", "id": id,
            "method": "textDocument/diagnostic", "params": { "textDocument": { "uri": uri } } });
        let me = Arc::clone(self);
        let answered = move |answer: Result<Value, LspError>| {
            let Some(items) = answer.ok().as_ref().and_then(full_items) else {
                return;
            };
            let fresh = me
                .state
                .lock()
                .is_ok_and(|mut s| s.pulled(&key, version, items));
            if fresh {
                me.publish(&key, &uri);
            }
        };
        if let Ok(mut pending) = self.pending.lock() {
            pending.insert(id, Box::new(answered));
        }
        let _ = self.writer.send(request.to_string());
    }

    #[note("pull-pull-on-push")]
    pub fn on_push(&self, payload: &str) -> bool {
        let Ok(val) = serde_json::from_str::<Value>(payload) else {
            return false;
        };
        if val.get("method").and_then(Value::as_str) != Some("textDocument/publishDiagnostics") {
            return false;
        }
        let uri = val.pointer("/params/uri").and_then(Value::as_str);
        let list = val.pointer("/params/diagnostics").and_then(Value::as_array);
        let (Some(uri), Some(list)) = (uri, list) else {
            return true;
        };
        let Some(file) = crate::uri_to_path(uri) else {
            return true;
        };
        let key = file_key(&file);
        if let Ok(mut s) = self.state.lock() {
            s.pushed(&key, list.clone());
        }
        self.publish(&key, uri);
        true
    }

    #[note("pull-pull-publish")]
    fn publish(&self, key: &str, uri: &str) {
        let merged = self.state.lock().map(|s| s.merged(key)).unwrap_or_default();
        if let Ok(mut kept) = self.published.lock() {
            kept.insert(key.to_string(), merged.clone());
        }
        let params = serde_json::json!({ "uri": uri, "diagnostics": merged });
        if let Ok(params) = serde_json::from_value(params) {
            let _ = self.diagnostics.send(params);
        }
        if let Some(w) = self.waker.lock().ok().as_deref().and_then(Option::as_ref) {
            w();
        }
    }
}

#[cfg(test)]
#[path = "test/pull/test.rs"]
mod tests;
