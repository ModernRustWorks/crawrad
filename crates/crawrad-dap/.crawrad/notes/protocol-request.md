---
id: protocol-request
kind: doc
target_file: src/protocol.rs
line_hint: 22
author: 
created: 
title: "Request"
tags: []
signature: "pub struct Request"
---

A DAP request message sent to or from the adapter.

A request — from the IDE to the adapter, or a reverse request from the
adapter (`runInTerminal`, `startDebugging`).

# Examples

```rust
use serde_json::json;
use crawrad_dap::Request;

let req = Request {
    seq: 1,
    command: "threads".into(),
    arguments: json!({}),
};
assert_eq!(req.command, "threads");
```
