---
id: tasks-taskmanager-cancel
kind: doc
target_file: src/tasks.rs
line_hint: 158
author: 
created: 
title: "TaskManager::cancel"
tags: []
signature: "pub fn cancel (& mut self , id : TaskId) -> bool"
---

Asks the job to stop by setting its cancelled flag.

Asks the job to stop (it reads `is_cancelled`). false if the id is not running.

# Examples

```rust
use crawrad_os::tasks::TaskManager;

let mut manager = TaskManager::<()>::new();
let id = manager.spawn("job", |r| {
    while !r.is_cancelled() {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
});
assert!(manager.cancel(id));
```
