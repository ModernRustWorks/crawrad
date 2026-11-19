---
id: route-route
kind: doc
target_file: src/route.rs
line_hint: 21
author: 
created: 
title: "Route"
tags: []
signature: "pub enum Route"
---

The bridge's decision for one line.

# Examples

```rust
use crawrad_mcp::route::Route;
use serde_json::json;

let r = Route::AnswerHere(json!({ "status": "error" }));
match r {
    Route::AnswerHere(val) => assert_eq!(val["status"], "error"),
    Route::Forward { .. } => unreachable!(),
}
```
