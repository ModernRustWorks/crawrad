//! MCP host tools: Build, Run & Diagnostics (build.* and run.*). The answers
//! are `crawrad_mcp::run_tool`'s; the shell finds the folder and audits.

use super::*;
use crawrad_host::HostModel;
use crawrad_mcp::cargo_tool;
use crawrad_mcp::protocol::ToolCallResult;
use crawrad_mcp::run_tool::{self, Answer};
use crawrad_note::note;
use serde_json::{Value, json};

impl McpServer {
    pub(super) fn tool_build_check(&mut self, host: &HostModel) -> ToolCallResult {
        let a = run_tool::in_project(host.workspace_root(), cargo_tool::check);
        self.audited("build.check", &json!({}), a)
    }

    pub(super) fn tool_build_build(&mut self, host: &HostModel, args: &Value) -> ToolCallResult {
        let a = run_tool::in_project(host.workspace_root(), |d| cargo_tool::build(d, args));
        self.audited("build.build", args, a)
    }

    pub(super) fn tool_build_get_diagnostics(&mut self, host: &HostModel) -> ToolCallResult {
        let a = run_tool::in_project(host.workspace_root(), cargo_tool::diagnostics);
        self.audited("build.get_diagnostics", &json!({}), a)
    }

    pub(super) fn tool_run_start(&mut self, host: &HostModel, args: &Value) -> ToolCallResult {
        let a = self.running_process.start(host.workspace_root(), args);
        self.audited("run.start", args, a)
    }

    pub(super) fn tool_run_stop(&mut self) -> ToolCallResult {
        let a = self.running_process.stop();
        self.audited("run.stop", &json!({}), a)
    }

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
