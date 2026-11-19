---
id: chat-config-as-json
kind: doc
target_file: src/chat_config.rs
line_hint: 108
author: 
created: 
title: "as_json"
tags: []
signature: "pub fn as_json (options : & [Value]) -> Value"
---

Formats ACP options into the MCP tool return value with groups flattened.

Formats ACP options into the MCP tool return value with groups flattened:
`[{id, name, category, current, options: [{value, name}]}]`.

# Examples

```rust
use crawrad_agent::chat_config::as_json;
use serde_json::json;

let opts = vec![json!({"id": "opt1", "name": "Option 1", "options": []})];
let val = as_json(&opts);
assert!(val.is_array());
```
