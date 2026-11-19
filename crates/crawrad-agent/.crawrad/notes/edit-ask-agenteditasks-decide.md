---
id: edit-ask-agenteditasks-decide
kind: doc
target_file: src/edit_ask.rs
line_hint: 117
author: 
created: 
title: "AgentEditAsks::decide"
tags: []
signature: "pub fn decide(&mut self, path: &Path, accepted: bool)"
---

Answers a file's held edit request.

Accepted answers the agent's allow-once option; rejected, its reject-once.

# Examples

```rust
use crawrad_agent::{AgentEditAsks, PermissionDecision, PermissionRequest};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

let (tx, rx) = std::sync::mpsc::channel();
let req = PermissionRequest::new(1, "s", "Edit", None, "", Vec::new(), tx, Duration::from_secs(9));
let mut asks = AgentEditAsks::default();
asks.hold(PathBuf::from("a.rs"), Arc::new(req));
asks.decide(Path::new("a.rs"), true);
assert_eq!(rx.try_recv().unwrap(), PermissionDecision::Selected("allow_once".into()));
```
