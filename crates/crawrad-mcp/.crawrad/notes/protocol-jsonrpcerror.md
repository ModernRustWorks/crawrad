---
id: protocol-jsonrpcerror
kind: doc
target_file: src/protocol.rs
line_hint: 68
author: 
created: 
title: "JsonRpcError"
tags: []
signature: "pub struct JsonRpcError"
---

JSON-RPC error payload.

# Examples

```rust
use crawrad_mcp::protocol::JsonRpcError;

let err = JsonRpcError {
    code: -32601,
    message: "Method not found".to_string(),
    data: None,
};
assert_eq!(err.code, -32601);
```
