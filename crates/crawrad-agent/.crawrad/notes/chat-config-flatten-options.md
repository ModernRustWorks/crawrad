---
id: chat-config-flatten-options
kind: doc
target_file: src/chat_config.rs
line_hint: 24
author: 
created: 
title: "flatten_options"
tags: []
signature: "pub fn flatten_options (options : & [Value]) -> Vec < (String , String) >"
---

Flattens an ACP options list recursively into (value, name) pairs.

Flattens an ACP options list, unpacking grouped options recursively into `(value, name)` pairs.

# Examples

```rust
use crawrad_agent::chat_config::flatten_options;
use serde_json::json;

let opts = vec![json!({"value": "opt1", "name": "Option 1"})];
let flat = flatten_options(&opts);
assert_eq!(flat.len(), 1);
assert_eq!(flat[0].0, "opt1");
```
