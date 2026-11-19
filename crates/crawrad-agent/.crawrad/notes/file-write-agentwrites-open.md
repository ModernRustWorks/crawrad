---
id: file-write-agentwrites-open
kind: doc
target_file: src/file_write.rs
line_hint: 78
author: 
created: 
title: "AgentWrites::open"
tags: []
signature: "pub fn open( &mut self, wake: impl Fn() + Send + 'static, ) -> impl FnMut(Arc<FileWriteRequest>) + Send + 'static"
---

A fresh channel for a new turn; returns the turn worker's forwarder.

Each write the forwarder sends calls `wake`, so the UI drains it.

# Examples

```rust
use crawrad_agent::{AgentWrites, FileWriteRequest};
use std::path::Path;
use std::sync::{Arc, Mutex};

let mut writes = AgentWrites::default();
let mut forward = writes.open(|| {});
forward(Arc::new(FileWriteRequest::new("a.rs".into(), String::new(), |_| {})));
assert_eq!(writes.drain().len(), 1);
```
