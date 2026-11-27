//! JSON-RPC 2.0 and MCP protocol data structures.

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[note("protocol-protocol-version")]
pub const PROTOCOL_VERSION: &str = "2024-11-05";

#[note("protocol-tools-version")]
pub const TOOLS_VERSION: &str = "1.0.0";

#[note("protocol-jsonrpcrequest")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

#[note("protocol-jsonrpcresponse")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Value, code: i64, message: impl Into<String>, data: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data,
            }),
        }
    }
}

/// Parses raw JSON-RPC request text, returning either the parsed request or an error response JSON.
pub fn parse_raw_request(trimmed: &str) -> Result<JsonRpcRequest, String> {
    serde_json::from_str(trimmed).map_err(|e| {
        let resp = JsonRpcResponse::error(Value::Null, -32700, format!("Parse error: {e}"), None);
        serde_json::to_string(&resp).unwrap_or_default()
    })
}

/// Checks the pending diff review guard when a save tool is called.
pub fn check_pending_review_save_guard(
    disable_gate: bool,
    name: &str,
    has_pending_review: bool,
) -> Result<(), crate::run_tool::Answer> {
    if !disable_gate && crate::tools::is_save_tool(name) && has_pending_review {
        let said = "Cannot save while a diff review is pending. Accept or reject the diff first.";
        return Err((
            ToolCallResult::error_text("pending-diff-review", said),
            "rejected",
        ));
    }
    Ok(())
}

#[note("protocol-initialize-result")]
pub fn initialize_result() -> Value {
    serde_json::json!({
        "protocolVersion": PROTOCOL_VERSION,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": { "name": "crawrad-ide", "version": "0.1.0" }
    })
}

#[note("protocol-jsonrpcerror")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[note("protocol-tooldefinition")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[note("protocol-toolcontent")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image {
        data: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
}

#[note("protocol-toolcallresult")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub content: Vec<ToolContent>,
    #[serde(rename = "isError", skip_serializing_if = "std::ops::Not::not")]
    pub is_error: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl ToolCallResult {
    pub fn success_text(text: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent::Text { text: text.into() }],
            is_error: false,
            data: None,
        }
    }

    pub fn success_json(data: Value, text: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent::Text { text: text.into() }],
            is_error: false,
            data: Some(data),
        }
    }

    #[note("protocol-toolcallresult-success-image")]
    pub fn success_image(
        png_base64: impl Into<String>,
        data: Value,
        text: impl Into<String>,
    ) -> Self {
        Self {
            content: vec![
                ToolContent::Image {
                    data: png_base64.into(),
                    mime_type: "image/png".to_string(),
                },
                ToolContent::Text { text: text.into() },
            ],
            is_error: false,
            data: Some(data),
        }
    }

    pub fn error_text(code: impl Into<String>, message: impl Into<String>) -> Self {
        let code = code.into();
        let message = message.into();
        let text = format!("[{code}] {message}");
        let data = serde_json::json!({
            "code": code,
            "message": message,
        });
        Self {
            content: vec![ToolContent::Text { text }],
            is_error: true,
            data: Some(data),
        }
    }
}
