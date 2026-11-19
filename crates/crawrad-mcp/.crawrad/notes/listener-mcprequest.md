---
id: listener-mcprequest
kind: doc
target_file: src/listener.rs
line_hint: 8
author: 
created: 
title: "McpRequest"
tags: []
signature: "pub struct McpRequest"
---

An incoming JSON-RPC 2.0 request frame dispatched from an IPC client.

# Examples

```rust
use crawrad_mcp::listener::McpRequest;
use std::sync::mpsc::channel;

let (tx, _rx) = channel();
let req = McpRequest {
    line: "{}".to_string(),
    reply_tx: tx,
};
assert_eq!(req.line, "{}");
```
