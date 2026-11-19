---
id: toolset-toolset-refuse-call
kind: doc
target_file: src/toolset.rs
line_hint: 157
author: 
created: 
title: "Toolset::refuse_call"
tags: []
signature: "pub fn refuse_call (& self , request : & Value) -> Option < Value >"
---

Refuses tool call requests that are outside this toolset.

The error a `tools/call` request outside this toolset gets, without
reaching the IDE; `None` lets the request through.

# Examples

```rust
use crawrad_mcp::toolset::Toolset;
use serde_json::json;

let ts = Toolset::parse("code").unwrap();
let req = json!({
    "method": "tools/call",
    "params": { "name": "designer.add" }
});
assert!(ts.refuse_call(&req).is_some());
```
