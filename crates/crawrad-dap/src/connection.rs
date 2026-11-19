//! One DAP conversation over any byte stream.
//!
//! A reader thread parses every message the adapter sends and routes it:
//!
//! - a **response** goes to whoever sent the request whose `seq` it names in
//!   `request_seq` — never to "the last request", because responses may come
//!   back in any order (`launch` answers after `configurationDone`);
//! - an **event** goes to the event channel;
//! - a **reverse request** (`runInTerminal`, `startDebugging`) is answered at
//!   once with `success: false`: an unanswered reverse request leaves the
//!   adapter waiting forever.
//!
//! When the stream ends, every request still waiting gets [`DapError::Closed`]
//! instead of hanging.

use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crawrad_framing::{TransportError, read_message, write_message};
use serde_json::Value;
use thiserror::Error;

use crate::protocol::{Event, ProtocolMessage, Request, Response};
use crawrad_note::note;

#[derive(Debug, Error)]
pub enum DapError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("transport error: {0}")]
    Transport(#[from] TransportError),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("`{command}` failed: {message}")]
    Failed { command: String, message: String },
    #[error("`{0}` timed out")]
    Timeout(String),
    #[error("the adapter closed the connection")]
    Closed,
    #[error("the adapter could not be started: {0}")]
    Spawn(String),
}

type Pending = Arc<Mutex<HashMap<i64, Sender<Response>>>>;
type SharedWriter = Arc<Mutex<Box<dyn Write + Send>>>;

#[note("connection-pendingresponse")]
pub struct PendingResponse {
    command: String,
    rx: Receiver<Response>,
}

impl PendingResponse {
    #[note("connection-pendingresponse-wait")]
    pub fn wait(self, timeout: Duration) -> Result<Value, DapError> {
        match self.rx.recv_timeout(timeout) {
            Ok(r) if r.success => Ok(r.body),
            Ok(r) => Err(DapError::Failed {
                command: self.command,
                message: r.message.unwrap_or_else(|| "no message".into()),
            }),
            Err(RecvTimeoutError::Timeout) => Err(DapError::Timeout(self.command)),
            Err(RecvTimeoutError::Disconnected) => Err(DapError::Closed),
        }
    }
}

pub struct DapConnection {
    writer: SharedWriter,
    next_seq: Arc<AtomicI64>,
    pending: Pending,
    events: Mutex<Receiver<Event>>,
    alive: Arc<AtomicBool>,
}

impl DapConnection {
    #[note("connection-dapconnection-new")]
    pub fn new<R, W>(reader: R, writer: W) -> Self
    where
        R: Read + Send + 'static,
        W: Write + Send + 'static,
    {
        let writer: SharedWriter = Arc::new(Mutex::new(Box::new(writer)));
        let next_seq = Arc::new(AtomicI64::new(1));
        let pending: Pending = Arc::new(Mutex::new(HashMap::new()));
        let alive = Arc::new(AtomicBool::new(true));
        let (events_tx, events_rx) = channel();
        {
            let writer = Arc::clone(&writer);
            let next_seq = Arc::clone(&next_seq);
            let pending = Arc::clone(&pending);
            let alive = Arc::clone(&alive);
            std::thread::Builder::new()
                .name("dap-reader".into())
                .spawn(move || {
                    reader_loop(reader, &writer, &next_seq, &pending, &events_tx);
                    alive.store(false, Ordering::SeqCst);
                    // Dropping the senders wakes every waiter with `Closed`.
                    if let Ok(mut p) = pending.lock() {
                        p.clear();
                    }
                })
                .expect("spawn the DAP reader thread");
        }
        Self {
            writer,
            next_seq,
            pending,
            events: Mutex::new(events_rx),
            alive,
        }
    }

    #[note("connection-dapconnection-is-alive")]
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }

    #[note("connection-dapconnection-send-request")]
    pub fn send_request(
        &self,
        command: &str,
        arguments: Value,
    ) -> Result<PendingResponse, DapError> {
        if !self.is_alive() {
            return Err(DapError::Closed);
        }
        let seq = self.next_seq.fetch_add(1, Ordering::SeqCst);
        let (tx, rx) = channel();
        self.pending.lock().expect("pending lock").insert(seq, tx);
        let message = ProtocolMessage::Request(Request {
            seq,
            command: command.to_string(),
            arguments,
        });
        if let Err(e) = send(&self.writer, &message) {
            self.pending.lock().expect("pending lock").remove(&seq);
            return Err(e);
        }
        Ok(PendingResponse {
            command: command.to_string(),
            rx,
        })
    }

    #[note("connection-dapconnection-request")]
    pub fn request(
        &self,
        command: &str,
        arguments: Value,
        timeout: Duration,
    ) -> Result<Value, DapError> {
        self.send_request(command, arguments)?.wait(timeout)
    }

    #[note("connection-dapconnection-next-event")]
    pub fn next_event(&self, timeout: Duration) -> Result<Event, DapError> {
        match self
            .events
            .lock()
            .expect("events lock")
            .recv_timeout(timeout)
        {
            Ok(e) => Ok(e),
            Err(RecvTimeoutError::Timeout) => Err(DapError::Timeout("event".into())),
            Err(RecvTimeoutError::Disconnected) => Err(DapError::Closed),
        }
    }

    #[note("connection-dapconnection-wait-event")]
    pub fn wait_event(&self, name: &str, timeout: Duration) -> Result<Event, DapError> {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            let left = deadline.saturating_duration_since(std::time::Instant::now());
            if left.is_zero() {
                return Err(DapError::Timeout(format!("event `{name}`")));
            }
            let e = self.next_event(left)?;
            if e.event == name {
                return Ok(e);
            }
        }
    }
}

fn send(writer: &SharedWriter, message: &ProtocolMessage) -> Result<(), DapError> {
    let payload = serde_json::to_string(message)?;
    let mut w = writer.lock().expect("writer lock");
    write_message(&mut *w, &payload)?;
    Ok(())
}

fn reader_loop<R: Read>(
    reader: R,
    writer: &SharedWriter,
    next_seq: &AtomicI64,
    pending: &Pending,
    events: &Sender<Event>,
) {
    let mut reader = BufReader::new(reader);
    loop {
        let Ok(payload) = read_message(&mut reader) else {
            return;
        };
        let Ok(message) = serde_json::from_str::<ProtocolMessage>(&payload) else {
            // A message this client cannot read is skipped, not fatal: the
            // next one may well be the answer somebody is waiting for.
            continue;
        };
        match message {
            ProtocolMessage::Response(r) => {
                let waiter = pending
                    .lock()
                    .ok()
                    .and_then(|mut p| p.remove(&r.request_seq));
                if let Some(tx) = waiter {
                    let _ = tx.send(r);
                }
            }
            ProtocolMessage::Event(e) => {
                let _ = events.send(e);
            }
            ProtocolMessage::Request(q) => {
                let answer = ProtocolMessage::Response(Response {
                    seq: next_seq.fetch_add(1, Ordering::SeqCst),
                    request_seq: q.seq,
                    success: false,
                    command: q.command,
                    message: Some("not supported by CrawRAD".into()),
                    body: Value::Null,
                });
                let _ = send(writer, &answer);
            }
        }
    }
}
