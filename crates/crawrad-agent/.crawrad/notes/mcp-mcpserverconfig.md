---
id: mcp-mcpserverconfig
kind: doc
target_file: src/mcp.rs
line_hint: 25
author: 
created: 
title: "McpServerConfig"
tags: []
signature: "pub struct McpServerConfig"
---

Configuration for a single connected MCP server.

# Examples

```rust
use crawrad_agent::{McpServerConfig, McpSource, McpTransport};
use std::collections::HashMap;

let config = McpServerConfig {
    name: "test".into(),
    transport: McpTransport::Stdio,
    command: "echo".into(),
    args: vec![],
    env: HashMap::new(),
    source: McpSource::Custom,
    enabled: true,
};
assert_eq!(config.name, "test");
```
