---
id: file-write-agentwrites-drain
kind: doc
target_file: src/file_write.rs
line_hint: 90
author: 
created: 
title: "AgentWrites::drain"
tags: []
signature: "pub fn drain(&self) -> Vec<Arc<FileWriteRequest>>"
---

The writes that arrived since the last call.

# Examples

```rust
use crawrad_agent::{AgentWrites, FileWriteRequest};
use std::path::Path;
use std::sync::{Arc, Mutex};

let mut writes = AgentWrites::default();
let _forward = writes.open(|| {});
assert!(writes.drain().is_empty());
```
