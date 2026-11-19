---
id: edit-ask-agenteditasks-hold
kind: doc
target_file: src/edit_ask.rs
line_hint: 110
author: 
created: 
title: "AgentEditAsks::hold"
tags: []
signature: "pub fn hold(&mut self, path: PathBuf, req: Arc<PermissionRequest>)"
---

Holds a file's edit request until its diff is decided.

A newer request on the same file rejects the older one.

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
assert!(rx.try_recv().is_err());
```
