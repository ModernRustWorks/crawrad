---
id: ide-app-mcp-host-mcpserver
kind: doc
target_file: src/ide_app/mcp_host.rs
line_hint: 33
author: 
created: 
title: "McpServer"
tags: []
signature: "pub struct McpServer"
---

The MCP server instance living in the IDE process.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let server = McpServer::new("editor");
assert_eq!(server.instance_id, "editor");
```
