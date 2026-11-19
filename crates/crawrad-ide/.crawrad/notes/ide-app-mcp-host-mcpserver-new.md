---
id: ide-app-mcp-host-mcpserver-new
kind: doc
target_file: src/ide_app/mcp_host.rs
line_hint: 54
author: 
created: 
title: "McpServer::new"
tags: []
signature: "pub fn new (instance_id : impl Into < String >) -> Self"
---

Creates a new MCP server for the given instance ID.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let server = McpServer::new("test");
assert_eq!(server.instance_id, "test");
```
