//! The adapter as a child process, talking DAP over its stdio.

use std::io::Read;
use std::path::Path;
use std::process::{Child, Stdio};
use std::sync::{Arc, Mutex};

use crate::connection::{DapConnection, DapError};
use crawrad_note::note;

#[note("client-dapclient")]
pub struct DapClient {
    connection: DapConnection,
    child: Child,
    stderr: Arc<Mutex<String>>,
}

impl DapClient {
    #[note("client-dapclient-spawn")]
    pub fn spawn(program: &Path, args: &[&str]) -> Result<Self, DapError> {
        let mut child = crawrad_os::process::no_console(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DapError::Spawn(format!("{}: {e}", program.display())))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| DapError::Spawn("no stdin".into()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| DapError::Spawn("no stdout".into()))?;
        // Stderr is drained in the background: a full pipe would block the
        // adapter, and what it says there is the only clue when it fails.
        let stderr = Arc::new(Mutex::new(String::new()));
        if let Some(mut err) = child.stderr.take() {
            let sink = Arc::clone(&stderr);
            std::thread::spawn(move || {
                let mut buf = [0u8; 4096];
                while let Ok(n) = err.read(&mut buf) {
                    if n == 0 {
                        break;
                    }
                    if let Ok(mut s) = sink.lock() {
                        s.push_str(&String::from_utf8_lossy(&buf[..n]));
                    }
                }
            });
        }
        Ok(Self {
            connection: DapConnection::new(stdout, stdin),
            child,
            stderr,
        })
    }

    pub fn connection(&self) -> &DapConnection {
        &self.connection
    }

    #[note("client-dapclient-stderr")]
    pub fn stderr(&self) -> String {
        self.stderr.lock().map(|s| s.clone()).unwrap_or_default()
    }
}

impl Drop for DapClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
