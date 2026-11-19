//! Audit logging for in-IDE MCP tool calls.
//!
//! Writes structured JSONL records of all tool calls (applied, rejected, error)
//! without blocking execution if disk writes fail.

use crawrad_note::note;
use serde::Serialize;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

#[note("audit-auditentry")]
#[derive(Debug, Clone, Serialize)]
pub struct AuditEntry {
    /// Timestamp (UTC epoch with millis).
    pub ts: String,
    /// Tool name called (e.g. "component.add", "code.read").
    pub tool: String,
    /// Arguments passed to the tool.
    pub args: Value,
    /// Outcome ("applied", "rejected", "error").
    pub outcome: String,
    /// Unique session ID.
    pub session_id: String,
}

pub struct AuditLogger;

impl AuditLogger {
    #[note("audit-auditlogger-log-path")]
    pub fn log_path() -> PathBuf {
        if let Ok(appdata) = std::env::var("APPDATA") {
            let p = PathBuf::from(appdata).join("CrawRAD").join("logs");
            let _ = std::fs::create_dir_all(&p);
            return p.join("mcp_audit.jsonl");
        }
        if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
            let p = PathBuf::from(home).join(".crawrad").join("logs");
            let _ = std::fs::create_dir_all(&p);
            return p.join("mcp_audit.jsonl");
        }
        std::env::temp_dir().join("crawrad_mcp_audit.jsonl")
    }

    #[note("audit-auditlogger-record")]
    pub fn record(tool: &str, args: &Value, outcome: &str, session_id: &str) {
        Self::record_to_path(&Self::log_path(), tool, args, outcome, session_id);
    }

    #[note("audit-auditlogger-record-to-path")]
    pub fn record_to_path(path: &Path, tool: &str, args: &Value, outcome: &str, session_id: &str) {
        let entry = AuditEntry {
            ts: current_timestamp(),
            tool: tool.to_string(),
            args: args.clone(),
            outcome: outcome.to_string(),
            session_id: session_id.to_string(),
        };

        if let Ok(json_line) = serde_json::to_string(&entry) {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                let _ = writeln!(file, "{json_line}");
            }
        }
    }
}

fn current_timestamp() -> String {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(dur) => {
            let secs = dur.as_secs();
            let millis = dur.subsec_millis();
            format!("{secs}.{millis:03}")
        }
        Err(_) => "0.000".to_string(),
    }
}
