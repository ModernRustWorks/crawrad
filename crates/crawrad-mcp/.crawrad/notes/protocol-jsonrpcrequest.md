---
id: protocol-jsonrpcrequest
kind: doc
target_file: src/protocol.rs
line_hint: 13
author: 
created: 
title: "JsonRpcRequest"
tags: []
signature: "pub struct JsonRpcRequest"
---

Incoming JSON-RPC request.

# Examples

```rust
use crawrad_mcp::protocol::JsonRpcRequest;

let req = JsonRpcRequest {
    jsonrpc: "2.0".to_string(),
    id: None,
    method: "tools/list".to_string(),
    params: None,
};
assert_eq!(req.method, "tools/list");
```
