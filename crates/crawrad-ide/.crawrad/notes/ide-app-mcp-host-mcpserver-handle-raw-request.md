---
id: ide-app-mcp-host-mcpserver-handle-raw-request
kind: doc
target_file: src/ide_app/mcp_host.rs
line_hint: 79
author: 
created: 
title: "McpServer::handle_raw_request"
tags: []
signature: "pub fn handle_raw_request (& mut self , app : & mut IdeApp , cx : & mut Cx , raw_line : & str) -> String"
---

Handles a raw JSON-RPC 2.0 text frame line on the IDE UI thread.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut server = McpServer::new("test");
let out = server.handle_raw_request(app, cx, "");
assert_eq!(out, "");
```
