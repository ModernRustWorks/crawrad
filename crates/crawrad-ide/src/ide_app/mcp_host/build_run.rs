//! MCP host tools: Build, Run & Diagnostics (build.* and run.*).
use super::*;
use crawrad_mcp::protocol::ToolCallResult;
use crawrad_mcp::run_tool::Answer;
use crawrad_note::note;
use serde_json::Value;

impl McpServer {
    #[note("ide-app-mcp-host-build-run-mcpserver-audited")]
    pub(super) fn audited(
        &mut self,
        tool: &str,
        args: &Value,
        (r, outcome): Answer,
    ) -> ToolCallResult {
        self.record_audit(tool, args, outcome);
        r
    }
}
