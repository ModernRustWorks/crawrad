---
id: controller-debugcontroller
kind: doc
target_file: src/controller.rs
line_hint: 122
author: 
created: 
title: "DebugController"
tags: []
signature: "pub struct DebugController"
---

The handle the IDE keeps while a session exists.

# Examples

```rust
use crawrad_dap::{AdapterText, DapError, DebugController};

let controller = DebugController::start(
    || Err(DapError::Closed),
    Box::new(AdapterText),
    || {},
);
assert!(controller.send(crawrad_dap::Command::Stop) || true);
```
