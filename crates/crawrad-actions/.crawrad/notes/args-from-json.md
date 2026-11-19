---
id: args-from-json
kind: doc
target_file: src/lib.rs
line_hint: 6029
author: 
created: 
title: "args_from_json"
tags: []
signature: "pub fn args_from_json (args : & serde_json :: Value) -> Args"
---

Converts a JSON-RPC arguments object into [`Args`].

An MCP call's JSON arguments as [`Args`]: each key's value with
[`value_from_json`].

# Examples

```rust
use crawrad_actions::args_from_json;

let json = serde_json::json!({ "node": "button1" });
let args = args_from_json(&json);
assert_eq!(args.as_text("node"), Some("button1"));
```
