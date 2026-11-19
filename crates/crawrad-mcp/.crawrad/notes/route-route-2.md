---
id: route-route-2
kind: doc
target_file: src/route.rs
line_hint: 34
author: 
created: 
title: "route"
tags: []
signature: "pub fn route (line : & str) -> Route"
---

Decides what to do with `line` (already trimmed, not empty).

# Examples

```rust
use crawrad_mcp::route::{route, Route};

let r = route(r#"{"jsonrpc":"2.0","method":"tools/list","id":1}"#);
match r {
    Route::Forward { expects_answer, .. } => assert!(expects_answer),
    Route::AnswerHere(_) => unreachable!(),
}
```
