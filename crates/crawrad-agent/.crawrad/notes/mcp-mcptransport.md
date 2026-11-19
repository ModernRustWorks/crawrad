---
id: mcp-mcptransport
kind: doc
target_file: src/mcp.rs
line_hint: 5
author: 
created: 
title: "McpTransport"
tags: []
signature: "pub enum McpTransport"
---

Communication transport for an MCP server.

# Examples

```rust
use crawrad_agent::McpTransport;

assert_eq!(McpTransport::default(), McpTransport::Stdio);
```
