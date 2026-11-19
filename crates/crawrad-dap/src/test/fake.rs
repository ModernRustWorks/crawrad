//! A fake debug adapter in-process, over `std::io::pipe`, for the tests of
//! the connection and of the session: each test scripts what the adapter says.

use std::io::{BufReader, PipeReader, PipeWriter};

use crawrad_framing::{read_message, write_message};
use serde_json::Value;

use crate::connection::DapConnection;
use crate::protocol::{Event, ProtocolMessage, Request, Response};

/// The adapter's end of the two pipes.
pub struct FakeAdapter {
    from_client: BufReader<PipeReader>,
    to_client: PipeWriter,
    seq: i64,
}

impl FakeAdapter {
    pub fn read_request(&mut self) -> Request {
        let text = read_message(&mut self.from_client).expect("a request");
        match serde_json::from_str(&text).expect("valid JSON") {
            ProtocolMessage::Request(r) => r,
            other => panic!("expected a request, got {other:?}"),
        }
    }
    pub fn read_response(&mut self) -> Response {
        let text = read_message(&mut self.from_client).expect("a response");
        match serde_json::from_str(&text).expect("valid JSON") {
            ProtocolMessage::Response(r) => r,
            other => panic!("expected a response, got {other:?}"),
        }
    }
    pub fn send(&mut self, message: ProtocolMessage) {
        write_message(
            &mut self.to_client,
            &serde_json::to_string(&message).unwrap(),
        )
        .unwrap();
    }
    pub fn send_raw(&mut self, payload: &str) {
        write_message(&mut self.to_client, payload).unwrap();
    }
    pub fn answer(&mut self, request: &Request, success: bool, message: Option<&str>, body: Value) {
        self.seq += 1;
        let r = Response {
            seq: self.seq,
            request_seq: request.seq,
            success,
            command: request.command.clone(),
            message: message.map(str::to_string),
            body,
        };
        self.send(ProtocolMessage::Response(r));
    }
    pub fn event(&mut self, name: &str, body: Value) {
        self.seq += 1;
        self.send(ProtocolMessage::Event(Event {
            seq: self.seq,
            event: name.into(),
            body,
        }));
    }
}

pub fn pair() -> (DapConnection, FakeAdapter) {
    let (adapter_reads, client_writes) = std::io::pipe().unwrap();
    let (client_reads, adapter_writes) = std::io::pipe().unwrap();
    let connection = DapConnection::new(client_reads, client_writes);
    let adapter = FakeAdapter {
        from_client: BufReader::new(adapter_reads),
        to_client: adapter_writes,
        seq: 1000,
    };
    (connection, adapter)
}
