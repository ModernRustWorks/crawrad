---
id: tasks-taskmanager-drain
kind: doc
target_file: src/tasks.rs
line_hint: 137
author: 
created: 
title: "TaskManager::drain"
tags: []
signature: "pub fn drain (& mut self) -> Vec < (TaskId , TaskEvent < T >) >"
---

Returns all events emitted by jobs since the last call.

Everything the jobs said since the last call, in order. Done/Failed remove the task from `running`; Progress updates TaskInfo.progress.

# Examples

```rust
use crawrad_os::tasks::{TaskEvent, TaskManager};

let mut manager = TaskManager::<i32>::new();
let id = manager.spawn("compute", |r| r.done(10));
// The job runs on its own thread: poll until it reports, with a generous deadline.
let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
let mut events = Vec::new();
while std::time::Instant::now() < deadline
    && !events.iter().any(|(tid, ev)| *tid == id && *ev == TaskEvent::Done(10))
{
    events.extend(manager.drain());
    std::thread::sleep(std::time::Duration::from_millis(5));
}
assert!(events.iter().any(|(tid, ev)| *tid == id && *ev == TaskEvent::Done(10)));
```
