---
id: file-write-agentwrites-decide
kind: doc
target_file: src/file_write.rs
line_hint: 101
author: 
created: 
title: "AgentWrites::decide"
tags: []
signature: "pub fn decide(&mut self, path: &Path, accepted: bool)"
---

The user accepted or rejected the edit to `path`: answer its agent.

# Examples

```rust
use crawrad_agent::{AgentWrites, FileWriteRequest};
use std::path::Path;
use std::sync::{Arc, Mutex};

let seen = Arc::new(Mutex::new(None));
let log = Arc::clone(&seen);
let mut writes = AgentWrites::default();
writes.hold("a.rs".into(), Arc::new(FileWriteRequest::new("a.rs".into(), String::new(), move |r| *log.lock().unwrap() = Some(r))));
writes.decide(Path::new("a.rs"), false);
assert!(seen.lock().unwrap().as_ref().unwrap().is_err());
```
