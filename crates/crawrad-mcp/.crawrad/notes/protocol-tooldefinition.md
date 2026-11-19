---
id: protocol-tooldefinition
kind: doc
target_file: src/protocol.rs
line_hint: 77
author: 
created: 
title: "ToolDefinition"
tags: []
signature: "pub struct ToolDefinition"
---

MCP Tool definition for `tools/list`.

# Examples

```rust
use crawrad_mcp::protocol::ToolDefinition;
use serde_json::json;

let tool = ToolDefinition {
    name: "test".to_string(),
    description: "A test tool".to_string(),
    input_schema: json!({ "type": "object" }),
};
assert_eq!(tool.name, "test");
```
