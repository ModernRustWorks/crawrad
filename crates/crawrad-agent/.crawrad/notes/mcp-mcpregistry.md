---
id: mcp-mcpregistry
kind: doc
target_file: src/mcp.rs
line_hint: 97
author: 
created: 
title: "McpRegistry"
tags: []
signature: "pub struct McpRegistry"
---

Registry of configured MCP servers.

# Examples

```rust
use crawrad_agent::McpRegistry;

let registry = McpRegistry::new();
assert!(!registry.servers.is_empty());
```
