---
id: client-auth-method-ids
kind: doc
target_file: src/client.rs
line_hint: 87
author: 
created: 
title: "auth_method_ids"
tags: []
signature: "pub fn auth_method_ids (initialize_result : & serde_json :: Value) -> Vec < String >"
---

The ids of the `authMethods` an `initialize` result offers, in order.

# Examples

```rust
use crawrad_agent::client::auth_method_ids;
use serde_json::json;

let init = json!({"authMethods": [{"id": "oauth"}]});
let ids = auth_method_ids(&init);
assert_eq!(ids, vec!["oauth"]);
```
