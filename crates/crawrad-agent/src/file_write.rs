//! Agent file write request models and deferred responses.
#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};

use crawrad_note::note;

type ResponderCallback = Box<dyn FnOnce(Result<(), String>) + Send>;

#[note("file-write-filewriterequest")]
pub struct FileWriteRequest {
    pub path: PathBuf,
    pub content: String,
    responder: Mutex<Option<ResponderCallback>>,
}

impl FileWriteRequest {
    #[note("file-write-filewriterequest-new")]
    pub fn new(
        path: PathBuf,
        content: String,
        responder: impl FnOnce(Result<(), String>) + Send + 'static,
    ) -> Self {
        Self {
            path,
            content,
            responder: Mutex::new(Some(Box::new(responder))),
        }
    }

    #[note("file-write-filewriterequest-respond")]
    pub fn respond(&self, result: Result<(), String>) {
        let callback = if let Ok(mut guard) = self.responder.lock() {
            guard.take()
        } else {
            None
        };
        if let Some(cb) = callback {
            cb(result);
        }
    }
}

impl Drop for FileWriteRequest {
    fn drop(&mut self) {
        self.respond(Err("The edit was not applied.".into()));
    }
}

impl fmt::Debug for FileWriteRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileWriteRequest")
            .field("path", &self.path)
            .field("content_length", &self.content.len())
            .finish()
    }
}

impl PartialEq for FileWriteRequest {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.content == other.content
    }
}

impl Eq for FileWriteRequest {}

#[note("file-write-agentwrites")]
#[derive(Default)]
pub struct AgentWrites {
    incoming: Option<Receiver<Arc<FileWriteRequest>>>,
    held: HashMap<PathBuf, Arc<FileWriteRequest>>,
}

impl AgentWrites {
    #[note("file-write-agentwrites-open")]
    pub fn open(
        &mut self,
        wake: impl Fn() + Send + 'static,
    ) -> impl FnMut(Arc<FileWriteRequest>) + Send + 'static {
        let (tx, rx) = channel();
        self.incoming = Some(rx);
        move |req| {
            let _ = tx.send(req);
            wake();
        }
    }
    #[note("file-write-agentwrites-drain")]
    pub fn drain(&self) -> Vec<Arc<FileWriteRequest>> {
        self.incoming
            .as_ref()
            .map(|rx| rx.try_iter().collect())
            .unwrap_or_default()
    }
    #[note("file-write-agentwrites-hold")]
    pub fn hold(&mut self, path: PathBuf, req: Arc<FileWriteRequest>) {
        self.held.insert(path, req);
    }
    #[note("file-write-agentwrites-decide")]
    pub fn decide(&mut self, path: &Path, accepted: bool) {
        if let Some(req) = self.held.remove(path) {
            req.respond(if accepted {
                Ok(())
            } else {
                Err("The user rejected the edit.".into())
            });
        }
    }
}

#[cfg(test)]
#[path = "test/file_write/test.rs"]
mod tests;
