---
id: controller-stop
kind: doc
target_file: src/controller.rs
line_hint: 71
author: 
created: 
title: "Stop"
tags: []
signature: "pub struct Stop"
---

Everything the IDE draws for a stop.

# Examples

```rust
use crawrad_dap::Stop;

let stop = Stop {
    thread_id: 1,
    reason: "breakpoint".into(),
    frames: Vec::new(),
    locals: Vec::new(),
};
assert_eq!(stop.thread_id, 1);
```
