use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ExtensionError;

pub const PROTOCOL_VERSION: &str = "2.0";

pub const METHOD_INITIALIZE: &str = "extension/initialize";
pub const METHOD_EXECUTE_COMMAND: &str = "extension/executeCommand";
pub const METHOD_PING: &str = "extension/ping";
pub const METHOD_PONG: &str = "extension/pong";
pub const METHOD_SHUTDOWN: &str = "extension/shutdown";
pub const METHOD_SHOW_MESSAGE: &str = "window/showMessage";

pub const METHOD_UI_RENDER_CARD: &str = "ui/renderCard";
pub const METHOD_UI_UPDATE_ELEMENT: &str = "ui/updateElement";
pub const METHOD_UI_CLOSE_CARD: &str = "ui/closeCard";
pub const METHOD_UI_EVENT: &str = "ui/event";

/// Method name for setting line marks on a file.
///
/// Line marks: the extension paints marks on a file's lines…
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::METHOD_EDITOR_SET_LINE_MARKS;
///
/// assert_eq!(METHOD_EDITOR_SET_LINE_MARKS, "editor/setLineMarks");
/// ```
pub const METHOD_EDITOR_SET_LINE_MARKS: &str = "editor/setLineMarks";

/// Notification method sent when a line mark icon is clicked.
///
/// …and is told when one's icon is clicked.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::METHOD_EDITOR_LINE_MARK_CLICKED;
///
/// assert_eq!(METHOD_EDITOR_LINE_MARK_CLICKED, "editor/lineMarkClicked");
/// ```
pub const METHOD_EDITOR_LINE_MARK_CLICKED: &str = "editor/lineMarkClicked";

/// Method name for setting an extension view tree.
///
/// Extension views: the extension fills a view's tree...
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::METHOD_VIEW_SET_TREE;
///
/// assert_eq!(METHOD_VIEW_SET_TREE, "view/setTree");
/// ```
pub const METHOD_VIEW_SET_TREE: &str = "view/setTree";

/// Method name for view item click notifications.
///
/// ...and is told when one of its items is clicked.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::METHOD_VIEW_ITEM_CLICKED;
///
/// assert_eq!(METHOD_VIEW_ITEM_CLICKED, "view/itemClicked");
/// ```
pub const METHOD_VIEW_ITEM_CLICKED: &str = "view/itemClicked";

/// A standard JSON-RPC 2.0 request frame exchanged over stdio or IPC.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::JsonRpcRequest;
///
/// let req = JsonRpcRequest::new_request(1, "test", None);
/// assert_eq!(req.id, Some(1));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl JsonRpcRequest {
    pub fn new_request(id: u64, method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: PROTOCOL_VERSION.to_string(),
            id: Some(id),
            method: method.into(),
            params,
        }
    }

    pub fn new_notification(method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: PROTOCOL_VERSION.to_string(),
            id: None,
            method: method.into(),
            params,
        }
    }

    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

/// JSON-RPC 2.0 error object.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::JsonRpcError;
///
/// let err = JsonRpcError::new(-32600, "Invalid Request");
/// assert_eq!(err.code, -32600);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcError {
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_data(code: i64, message: impl Into<String>, data: Value) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
        }
    }
}

/// A standard JSON-RPC 2.0 response frame.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::JsonRpcResponse;
///
/// let resp = JsonRpcResponse::success(Some(1), serde_json::json!({}));
/// assert!(resp.is_success());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    pub fn success(id: Option<u64>, result: Value) -> Self {
        Self {
            jsonrpc: PROTOCOL_VERSION.to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Option<u64>, code: i64, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: PROTOCOL_VERSION.to_string(),
            id,
            result: None,
            error: Some(JsonRpcError::new(code, message)),
        }
    }

    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }
}

/// Parameters sent during the `extension/initialize` handshake.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::InitializeParams;
///
/// let params = InitializeParams {
///     extension_id: "ext.sample".to_string(),
///     workspace_path: None,
/// };
/// assert_eq!(params.extension_id, "ext.sample");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitializeParams {
    pub extension_id: String,
    #[serde(default)]
    pub workspace_path: Option<String>,
}

/// Response returned from `extension/initialize`.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::InitializeResult;
///
/// let result = InitializeResult {
///     success: true,
///     commands: vec!["ext.sample.run".to_string()],
/// };
/// assert!(result.success);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitializeResult {
    pub success: bool,
    #[serde(default)]
    pub commands: Vec<String>,
}

/// Parameters for `extension/executeCommand`.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::ExecuteCommandParams;
///
/// let params = ExecuteCommandParams {
///     command: "my_ext.action".to_string(),
///     arguments: None,
/// };
/// assert_eq!(params.command, "my_ext.action");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecuteCommandParams {
    pub command: String,
    #[serde(default)]
    pub arguments: Option<Value>,
}

/// Result returned from `extension/executeCommand`.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::ExecuteCommandResult;
///
/// let res = ExecuteCommandResult {
///     success: true,
///     result: None,
///     error: None,
/// };
/// assert!(res.success);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecuteCommandResult {
    pub success: bool,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub error: Option<String>,
}

/// Notification payload for `window/showMessage`.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::ShowMessageNotification;
///
/// let notif = ShowMessageNotification {
///     level: "info".to_string(),
///     message: "Ready".to_string(),
/// };
/// assert_eq!(notif.level, "info");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShowMessageNotification {
    pub level: String,
    pub message: String,
}

/// Message received from the external extension process.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::{IncomingMessage, JsonRpcRequest};
///
/// let msg = IncomingMessage::Request(JsonRpcRequest::new_notification("ping", None));
/// match msg {
///     IncomingMessage::Request(req) => assert!(req.is_notification()),
///     IncomingMessage::Response(_) => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomingMessage {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
}

/// Parse a raw incoming line from the process stdio pipe.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::protocol::{parse_message, IncomingMessage};
///
/// let line = r#"{"jsonrpc":"2.0","method":"ping"}"#;
/// let msg = parse_message(line).unwrap();
/// assert!(matches!(msg, IncomingMessage::Request(_)));
/// ```
///
/// # Errors
///
/// Returns [`ExtensionError::ProtocolError`] if the line is empty, not valid JSON, or missing required RPC fields.
pub fn parse_message(line: &str) -> Result<IncomingMessage, ExtensionError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(ExtensionError::ProtocolError("empty message line".into()));
    }

    let val: Value = serde_json::from_str(trimmed)
        .map_err(|e| ExtensionError::ProtocolError(format!("malformed json: {e}")))?;

    if val.get("method").is_some() {
        let req: JsonRpcRequest = serde_json::from_value(val)
            .map_err(|e| ExtensionError::ProtocolError(format!("invalid request: {e}")))?;
        Ok(IncomingMessage::Request(req))
    } else {
        let resp: JsonRpcResponse = serde_json::from_value(val)
            .map_err(|e| ExtensionError::ProtocolError(format!("invalid response: {e}")))?;
        Ok(IncomingMessage::Response(resp))
    }
}
