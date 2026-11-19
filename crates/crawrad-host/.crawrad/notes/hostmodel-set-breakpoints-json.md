---
id: hostmodel-set-breakpoints-json
kind: doc
target_file: src/lib.rs
line_hint: 264
author: 
created: 
title: "HostModel::set_breakpoints_json"
tags: []
signature: "pub fn set_breakpoints_json (& mut self , args : & serde_json :: Value , root : Option < & std :: path :: Path > ,) -> Result < serde_json :: Value , String >"
---

Sets breakpoints from a JSON payload for the MCP tool.

The MCP's `debug.set_breakpoints`: `{path, lines}`, the path absolute
or relative to `root` (the project group).

# Examples

```rust
use crawrad_host::HostModel;
use serde_json::json;

let mut host = HostModel::new();
let res = host.set_breakpoints_json(&json!({"path": "src/lib.rs", "lines": [5]}), None);
assert!(res.is_ok());
```

# Errors

Returns an error message if the payload is missing `path` or `lines`.
