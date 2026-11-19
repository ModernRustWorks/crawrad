---
id: acp-registry-acptoolcall
kind: doc
target_file: src/acp_registry.rs
line_hint: 1032
author: 
created: 
title: "AcpToolCall"
tags: []
signature: "pub struct AcpToolCall"
---

Result and audit outcome of an ACP tool invocation.

MCP tool handler for `acp.list_agents`.
What one `acp.*` MCP call did: its result, the audit word, and an install
it started (the shell watches that one; nothing else touches the window).

# Examples

```rust
use crawrad_agent::acp_registry::AcpToolCall;
use crawrad_mcp::protocol::ToolCallResult;

let call = AcpToolCall {
    result: ToolCallResult::success_text("ok"),
    audit: "applied",
    install: None,
};
assert_eq!(call.audit, "applied");
```
