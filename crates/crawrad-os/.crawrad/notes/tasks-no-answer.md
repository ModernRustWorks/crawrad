---
id: tasks-no-answer
kind: doc
target_file: src/tasks.rs
line_hint: 10
author: 
created: 
title: "NO_ANSWER"
tags: []
signature: "pub const NO_ANSWER : & str = \"the task ended without answering\""
---

What a job that returned (or panicked) without `done`/`failed` reports.

# Examples

```rust
use crawrad_os::tasks::NO_ANSWER;

assert_eq!(NO_ANSWER, "the task ended without answering");
```
