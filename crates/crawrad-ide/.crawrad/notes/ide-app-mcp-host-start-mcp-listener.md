---
id: ide-app-mcp-host-start-mcp-listener
kind: doc
target_file: src/ide_app/mcp_host.rs
line_hint: 26
author: 
created: 
title: "start_mcp_listener"
tags: []
signature: "pub fn start_mcp_listener (instance_id : & str ,) -> std :: io :: Result < std :: sync :: mpsc :: Receiver < McpRequest > >"
---

Starts the in-process MCP listener for `instance_id`.

Starts the in-process MCP listener for `instance_id`; each request wakes
the window.

# Errors

Returns an error if the IPC channel or socket listener fails to initialize.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let rx = start_mcp_listener("test-instance");
assert!(rx.is_ok());
```
