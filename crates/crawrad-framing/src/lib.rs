//! Framing for messages over a byte stream: a `Content-Length` header, a blank
//! line, then exactly that many bytes of payload.
//!
//! It is the base protocol of LSP and of DAP alike, which is why it lives on its
//! own: `crawrad-lsp` and `crawrad-dap` both frame through here instead of each
//! keeping a copy.
#![forbid(unsafe_code)]

use crawrad_note::note;
use std::io::{self, BufRead, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Protocol error: missing Content-Length header")]
    MissingContentLength,
    #[error("Protocol error: invalid Content-Length value '{0}'")]
    InvalidContentLength(String),
    #[error("JSON serialization or deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Stream closed unexpectedly")]
    UnexpectedEof,
}

#[note("write-message")]
pub fn write_message<W: Write>(writer: &mut W, payload: &str) -> Result<(), TransportError> {
    let header = format!("Content-Length: {}\r\n\r\n", payload.len());
    writer.write_all(header.as_bytes())?;
    writer.write_all(payload.as_bytes())?;
    writer.flush()?;
    Ok(())
}

#[note("read-message")]
pub fn read_message<R: BufRead>(reader: &mut R) -> Result<String, TransportError> {
    let mut content_length: Option<usize> = None;

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            return Err(TransportError::UnexpectedEof);
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            // End of headers
            break;
        }

        if let Some(rest) = trimmed.strip_prefix("Content-Length:") {
            let len_str = rest.trim();
            let parsed_len: usize = len_str
                .parse()
                .map_err(|_| TransportError::InvalidContentLength(len_str.to_string()))?;
            content_length = Some(parsed_len);
        }
        // Additional headers (e.g. Content-Type) are ignored as per LSP spec
    }

    let len = content_length.ok_or(TransportError::MissingContentLength)?;
    let mut buffer = vec![0u8; len];
    reader.read_exact(&mut buffer)?;

    let payload =
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_and_read_roundtrip() {
        let payload = r#"{"jsonrpc":"2.0","method":"test","params":{"hello":"world"}}"#;
        let mut buffer = Vec::new();

        write_message(&mut buffer, payload).expect("write failed");

        let mut cursor = Cursor::new(buffer);
        let read = read_message(&mut cursor).expect("read failed");
        assert_eq!(read, payload);
    }

    #[test]
    fn test_multiple_messages_in_stream() {
        let p1 = r#"{"id":1,"result":"ok"}"#;
        let p2 = r#"{"method":"notify","params":{}}"#;

        let mut buffer = Vec::new();
        write_message(&mut buffer, p1).unwrap();
        write_message(&mut buffer, p2).unwrap();

        let mut cursor = Cursor::new(buffer);
        let r1 = read_message(&mut cursor).unwrap();
        let r2 = read_message(&mut cursor).unwrap();

        assert_eq!(r1, p1);
        assert_eq!(r2, p2);
    }
}
