//! The DAP messages, as they travel.
//!
//! Only the envelope is typed here. `arguments` and `body` stay as JSON values:
//! each command has its own shape, and typing them one by one belongs to the
//! session, which knows which commands it sends. The one body
//! type typed now is [`Variable`], because it is what a [`crate::ValuePresenter`]
//! receives.

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[note("protocol-protocolmessage")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProtocolMessage {
    Request(Request),
    Response(Response),
    Event(Event),
}

#[note("protocol-request")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub seq: i64,
    pub command: String,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub arguments: Value,
}

#[note("protocol-response")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub seq: i64,
    pub request_seq: i64,
    pub success: bool,
    pub command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub body: Value,
}

#[note("protocol-event")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub seq: i64,
    pub event: String,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub body: Value,
}

#[note("protocol-variable")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    pub name: String,
    pub value: String,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// Non-zero when the variable has children to ask for.
    #[serde(default)]
    pub variables_reference: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluate_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_reference: Option<String>,
}
