---
id: client-is-auth-required
kind: doc
target_file: src/client.rs
line_hint: 98
author: 
created: 
title: "is_auth_required"
tags: []
signature: "pub fn is_auth_required (error : & str) -> bool"
---

Determines whether a JSON-RPC error indicates authentication required.

Is a JSON-RPC error the protocol's "authentication required" (code -32000)?

# Examples

```rust
use crawrad_agent::client::is_auth_required;

assert!(is_auth_required(r#"{"code": -32000}"#));
assert!(!is_auth_required(r#"{"code": -32600}"#));
```
