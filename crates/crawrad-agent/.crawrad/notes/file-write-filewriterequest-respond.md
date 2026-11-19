---
id: file-write-filewriterequest-respond
kind: doc
target_file: src/file_write.rs
line_hint: 33
author: 
created: 
title: "FileWriteRequest::respond"
tags: []
signature: "pub fn respond (& self , result : Result < () , String >)"
---

Answers the agent: `Ok` when the edit landed, `Err` with the reason when not.

Only the first answer is sent; later ones do nothing.

# Examples

```rust
use crawrad_agent::FileWriteRequest;
use std::sync::{Arc, Mutex};

let seen = Arc::new(Mutex::new(Vec::new()));
let log = Arc::clone(&seen);
let req = FileWriteRequest::new("a.rs".into(), String::new(), move |r| log.lock().unwrap().push(r));
req.respond(Ok(()));
req.respond(Err("late".into()));
assert_eq!(seen.lock().unwrap().len(), 1);
```
