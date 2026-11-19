---
id: protocol-response
kind: doc
target_file: src/protocol.rs
line_hint: 31
author: 
created: 
title: "Response"
tags: []
signature: "pub struct Response"
---

The answer to a request. `request_seq` — not `seq` — names the request.

# Examples

```rust
use serde_json::Value;
use crawrad_dap::Response;

let resp = Response {
    seq: 2,
    request_seq: 1,
    success: true,
    command: "threads".into(),
    message: None,
    body: Value::Null,
};
assert!(resp.success);
```
