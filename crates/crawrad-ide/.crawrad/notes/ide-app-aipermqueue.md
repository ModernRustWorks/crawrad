---
id: ide-app-aipermqueue
kind: doc
target_file: src/ide_app.rs
line_hint: 360
author: 
created: 
title: "AiPermQueue"
tags: []
signature: "pub type AiPermQueue = crawrad_agent :: PermissionQueue"
---

Permission request queue.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let queue = AiPermQueue::default();
assert!(queue.is_empty());
```
