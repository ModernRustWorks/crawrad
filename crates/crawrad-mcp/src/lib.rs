//! Model Context Protocol (MCP) data structures, action adapters, and stdio bridge for CrawRAD.
#![forbid(unsafe_code)]

pub mod audit;
pub mod capture_tool;
pub mod cargo_tool;
pub mod component_tool;
pub mod designer_tool;
pub mod dialog_tool;
pub mod dialogs;
pub mod dialogs_table;
pub mod edit;
pub mod form_json;
pub mod listener;
pub mod paging;
pub mod project_tool;
pub mod project_tree;
pub mod properties;
pub mod protocol;
pub mod route;
pub mod run_tool;
pub mod strings_tool;
pub mod tools;
pub mod toolset;
pub mod view_tool;
pub mod workspace_config;

pub use audit::{AuditEntry, AuditLogger};
pub use protocol::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, PROTOCOL_VERSION, TOOLS_VERSION, ToolCallResult,
    ToolDefinition,
};
pub use tools::{build_tools_list, build_tools_list_with_registry, code_writing_tool_names};
pub use toolset::Toolset;
