---
id: tools-build-tools-list-with-registry
kind: doc
target_file: src/tools.rs
line_hint: 14
author: 
created: 
title: "build_tools_list_with_registry"
tags: []
signature: "pub fn build_tools_list_with_registry (registry : Option < & CapabilityRegistry > ,) -> Vec < ToolDefinition >"
---

Builds the list of MCP tools enriched with dynamic capabilities.

Builds the complete list of MCP tools exposed by the IDE, enriched with
dynamic capabilities and extension tools from CapabilityRegistry.

# Examples

```rust
use crawrad_mcp::tools::build_tools_list_with_registry;

let tools = build_tools_list_with_registry(None);
assert!(!tools.is_empty());
```
