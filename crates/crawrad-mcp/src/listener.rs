//! The in-process MCP listener (moved out of the shell): it
//! accepts IPC connections for an IDE instance and hands every request line
//! to the window with a channel for the answer.

use crawrad_note::note;
use std::io::{BufRead, Write};

#[note("listener-mcprequest")]
pub struct McpRequest {
    pub line: String,
    pub reply_tx: std::sync::mpsc::Sender<String>,
}

#[note("listener-start")]
pub fn start(
    instance_id: &str,
    wake: fn(),
) -> std::io::Result<std::sync::mpsc::Receiver<McpRequest>> {
    let listener = crawrad_os::ipc::IpcListener::bind(instance_id)?;
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        while let Ok(mut stream) = listener.accept() {
            let tx_conn = tx.clone();
            std::thread::spawn(move || {
                let reader_stream = match stream.try_clone() {
                    Ok(s) => s,
                    Err(_) => return,
                };
                let mut reader = std::io::BufReader::new(reader_stream);
                let mut line = String::new();
                while let Ok(n) = reader.read_line(&mut line) {
                    if n == 0 {
                        break;
                    }
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let (reply_tx, reply_rx) = std::sync::mpsc::channel();
                        if tx_conn
                            .send(McpRequest {
                                line: trimmed.to_string(),
                                reply_tx,
                            })
                            .is_err()
                        {
                            break;
                        }
                        wake();
                        if let Ok(resp) = reply_rx.recv()
                            && !resp.is_empty()
                        {
                            let _ = stream.write_all(resp.as_bytes());
                            let _ = stream.write_all(b"\n");
                            let _ = stream.flush();
                        }
                    }
                    line.clear();
                }
            });
        }
    });
    Ok(rx)
}
