---
id: tools-unwrap-action-execute
kind: doc
target_file: src/tools.rs
line_hint: 1192
author: 
created: 
title: "unwrap_action_execute"
tags: []
signature: "pub fn unwrap_action_execute < 'a > (name : & 'a str , args : & 'a Value) -> (& 'a str , & 'a Value)"
---

Unwraps action.execute payloads into their target tool name and arguments.

`action.execute` carries the real tool in `name` and its arguments in
`arguments` (or `params`); any other call is returned as it came.

# Examples

```rust
use crawrad_mcp::tools::unwrap_action_execute;
use serde_json::json;

let args = json!({ "name": "form.read", "arguments": {} });
let (target, inner) = unwrap_action_execute("action.execute", &args);
assert_eq!(target, "form.read");
```
