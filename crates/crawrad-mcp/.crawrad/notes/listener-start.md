---
id: listener-start
kind: doc
target_file: src/listener.rs
line_hint: 14
author: 
created: 
title: "start"
tags: []
signature: "pub fn start (instance_id : & str , wake : fn () ,) -> std :: io :: Result < std :: sync :: mpsc :: Receiver < McpRequest > >"
---

Starts the listener thread for an IDE instance to receive MCP requests.

Starts the listener thread for `instance_id`: every line a client sends
becomes a [`McpRequest`], and `wake` is called so the window reads it.

# Examples

```rust
use crawrad_mcp::listener::start;

fn dummy_wake() {}

let id = format!("test-listener-{}", std::process::id());
let rx = start(&id, dummy_wake);
assert!(rx.is_ok());
```

# Errors

Returns `Err` if binding the IPC listener fails.
