---
id: edit-ask-option-of
kind: doc
target_file: src/edit_ask.rs
line_hint: 89
author: 
created: 
title: "option_of"
tags: []
signature: "pub fn option_of(req: &PermissionRequest, accepted: bool) -> String"
---

The agent's own allow-once or reject-once option id.

Falls back to `allow_once` / `reject_once` when the agent sent none.

# Examples

```rust
use crawrad_agent::PermissionRequest;
use crawrad_agent::edit_ask::option_of;
use std::time::Duration;

let (tx, _rx) = std::sync::mpsc::channel();
let req = PermissionRequest::new(1, "s", "Edit", None, "", Vec::new(), tx, Duration::from_secs(9));
assert_eq!(option_of(&req, false), "reject_once");
```
