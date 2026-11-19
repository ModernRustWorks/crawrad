---
id: protocol-jsonrpcresponse
kind: doc
target_file: src/protocol.rs
line_hint: 24
author: 
created: 
title: "JsonRpcResponse"
tags: []
signature: "pub struct JsonRpcResponse"
---

Outgoing JSON-RPC response.

# Examples

```rust
use crawrad_mcp::protocol::JsonRpcResponse;
use serde_json::json;

let resp = JsonRpcResponse::success(json!(1), json!({ "status": "ok" }));
assert_eq!(resp.jsonrpc, "2.0");
assert!(resp.error.is_none());
```
