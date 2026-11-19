---
id: file-write-agentwrites-hold
kind: doc
target_file: src/file_write.rs
line_hint: 97
author: 
created: 
title: "AgentWrites::hold"
tags: []
signature: "pub fn hold(&mut self, path: PathBuf, req: Arc<FileWriteRequest>)"
---

Keeps `req` unanswered while the user reviews the edit to `path`.

# Examples

```rust
use crawrad_agent::{AgentWrites, FileWriteRequest};
use std::path::Path;
use std::sync::{Arc, Mutex};

let mut writes = AgentWrites::default();
writes.hold("a.rs".into(), Arc::new(FileWriteRequest::new("a.rs".into(), String::new(), |_| {})));
```
