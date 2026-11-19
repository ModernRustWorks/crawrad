---
id: edit-ask-agenteditasks-allow
kind: doc
target_file: src/edit_ask.rs
line_hint: 117
author: 
created: 
title: "AgentEditAsks::allow"
tags: []
signature: "pub fn allow(&self, req: &PermissionRequest)"
---

Allows an edit request at once, with the agent's allow-once option.

Used when the review mode applies edits directly, so no diff waits.

# Examples

```rust
use crawrad_agent::{AgentEditAsks, PermissionDecision, PermissionRequest};
use std::time::Duration;

let (tx, rx) = std::sync::mpsc::channel();
let req = PermissionRequest::new(1, "s", "Edit", None, "", Vec::new(), tx, Duration::from_secs(9));
AgentEditAsks::default().allow(&req);
assert_eq!(rx.try_recv().unwrap(), PermissionDecision::Selected("allow_once".into()));
```
