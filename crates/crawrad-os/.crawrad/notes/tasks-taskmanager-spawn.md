---
id: tasks-taskmanager-spawn
kind: doc
target_file: src/tasks.rs
line_hint: 110
author: 
created: 
title: "TaskManager::spawn"
tags: []
signature: "pub fn spawn (& mut self , kind : & 'static str , job : impl FnOnce (Reporter < T >) + Send + 'static ,) -> TaskId"
---

Runs `job` on its own thread.

The ONE place a background job is registered.

# Examples

```rust
use crawrad_os::tasks::TaskManager;

let mut manager = TaskManager::<i32>::new();
let id = manager.spawn("calc", |reporter| {
    reporter.done(42);
});
assert_eq!(manager.running().len(), 1);
```
