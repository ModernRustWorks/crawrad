//! How a raw value becomes readable text — one presenter per language.
//!
//! `crawrad-dap` stops at the raw tree the adapter sends. Turning
//! `alloc::string::String` into `"Olá"` is language knowledge, and it lives
//! outside this crate (ADR 0048 §3.2): a Rust presenter in `crawrad-dap-rust`,
//! and any other language in its own crate, without touching the client.

use std::time::Duration;

use serde_json::{Value, json};

use crate::base64;
use crate::connection::{DapConnection, DapError};
use crate::protocol::Variable;
use crawrad_note::note;

#[note("presenter-valuesource")]
pub trait ValueSource {
    #[note("presenter-valuesource-children")]
    fn children(&self, variables_reference: i64) -> Result<Vec<Variable>, DapError>;

    #[note("presenter-valuesource-read-memory")]
    fn read_memory(&self, memory_reference: &str, count: usize) -> Result<Vec<u8>, DapError>;

    #[note("presenter-valuesource-evaluate")]
    fn evaluate(&self, expression: &str, frame_id: Option<i64>) -> Result<Variable, DapError>;
}

#[note("presenter-valuepresenter")]
pub trait ValuePresenter: Send + Sync {
    #[note("presenter-valuepresenter-present")]
    fn present(&self, variable: &Variable, source: &dyn ValueSource) -> Option<String>;
}

#[note("presenter-adaptertext")]
pub struct AdapterText;

impl ValuePresenter for AdapterText {
    fn present(&self, _variable: &Variable, _source: &dyn ValueSource) -> Option<String> {
        None
    }
}

#[note("presenter-question-timeout")]
const QUESTION_TIMEOUT: Duration = Duration::from_secs(5);

impl ValueSource for DapConnection {
    fn children(&self, variables_reference: i64) -> Result<Vec<Variable>, DapError> {
        let body = self.request(
            "variables",
            json!({ "variablesReference": variables_reference }),
            QUESTION_TIMEOUT,
        )?;
        Ok(serde_json::from_value(
            body.get("variables")
                .cloned()
                .unwrap_or(Value::Array(vec![])),
        )?)
    }

    fn read_memory(&self, memory_reference: &str, count: usize) -> Result<Vec<u8>, DapError> {
        let body = self.request(
            "readMemory",
            json!({ "memoryReference": memory_reference, "count": count }),
            QUESTION_TIMEOUT,
        )?;
        let data = body.get("data").and_then(Value::as_str).unwrap_or("");
        base64::decode(data).ok_or_else(|| DapError::Failed {
            command: "readMemory".into(),
            message: "the data is not base64".into(),
        })
    }

    fn evaluate(&self, expression: &str, frame_id: Option<i64>) -> Result<Variable, DapError> {
        let mut args = json!({ "expression": expression, "context": "watch" });
        if let Some(id) = frame_id {
            args["frameId"] = json!(id);
        }
        let body = self.request("evaluate", args, QUESTION_TIMEOUT)?;
        Ok(Variable {
            name: expression.to_string(),
            value: body
                .get("result")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            type_name: body.get("type").and_then(Value::as_str).map(str::to_string),
            variables_reference: body
                .get("variablesReference")
                .and_then(Value::as_i64)
                .unwrap_or(0),
            evaluate_name: Some(expression.to_string()),
            memory_reference: body
                .get("memoryReference")
                .and_then(Value::as_str)
                .map(str::to_string),
        })
    }
}

#[note("presenter-framescoped")]
pub struct FrameScoped<'a> {
    pub source: &'a dyn ValueSource,
    pub frame_id: i64,
}

impl ValueSource for FrameScoped<'_> {
    fn children(&self, variables_reference: i64) -> Result<Vec<Variable>, DapError> {
        self.source.children(variables_reference)
    }

    fn read_memory(&self, memory_reference: &str, count: usize) -> Result<Vec<u8>, DapError> {
        self.source.read_memory(memory_reference, count)
    }

    fn evaluate(&self, expression: &str, frame_id: Option<i64>) -> Result<Variable, DapError> {
        self.source
            .evaluate(expression, frame_id.or(Some(self.frame_id)))
    }
}
