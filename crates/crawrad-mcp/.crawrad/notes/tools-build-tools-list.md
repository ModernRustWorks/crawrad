---
id: tools-build-tools-list
kind: doc
target_file: src/tools.rs
line_hint: 9
author: 
created: 
title: "build_tools_list"
tags: []
signature: "pub fn build_tools_list () -> Vec < ToolDefinition >"
---

Builds the complete list of MCP tools exposed by the IDE.

Builds the complete list of MCP tools exposed by the IDE without extension registry.

# Examples

```rust
use crawrad_mcp::tools::build_tools_list;

let tools = build_tools_list();
assert!(!tools.is_empty());
```
