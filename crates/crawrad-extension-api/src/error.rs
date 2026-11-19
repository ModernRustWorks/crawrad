use thiserror::Error;

/// Errors produced when validating or executing extension lifecycle operations.
///
/// # Examples
///
/// ```rust
/// use crawrad_extension_api::ExtensionError;
///
/// let err = ExtensionError::NotFound("manifest.toml".to_string());
/// assert_eq!(err.to_string(), "resource not found: manifest.toml");
/// ```
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExtensionError {
    #[error("invalid extension manifest: {0}")]
    InvalidManifest(String),

    #[error("extension activation failed: {0}")]
    ActivationFailed(String),

    #[error("extension deactivation failed: {0}")]
    DeactivationFailed(String),

    #[error("command registration failed for '{id}': {reason}")]
    CommandRegistrationFailed { id: String, reason: String },

    #[error("contribution conflict: {0}")]
    Conflict(String),

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("command execution failed: {0}")]
    ExecutionFailed(String),

    #[error("duplicate extension or resource: {0}")]
    Duplicate(String),

    #[error("io error: {0}")]
    Io(String),

    #[error("process error: {0}")]
    ProcessError(String),

    #[error("extension process crashed for '{extension_id}' (exit code: {exit_code:?})")]
    ProcessCrashed {
        extension_id: String,
        exit_code: Option<i32>,
    },

    #[error("ipc error: {0}")]
    IpcError(String),

    #[error("protocol error: {0}")]
    ProtocolError(String),

    #[error("internal error: {0}")]
    Internal(String),
}
