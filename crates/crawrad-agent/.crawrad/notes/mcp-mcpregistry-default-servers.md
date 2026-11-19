---
id: mcp-mcpregistry-default-servers
kind: doc
target_file: src/mcp.rs
line_hint: 110
author: 
created: 
title: "McpRegistry::default_servers"
tags: []
signature: "pub fn default_servers () -> Vec < McpServerConfig >"
---

Built-in default MCP servers provided with CrawRAD.

# Examples

```rust
use crawrad_agent::McpRegistry;

let servers = McpRegistry::default_servers();
assert_eq!(servers.len(), 2);
```
