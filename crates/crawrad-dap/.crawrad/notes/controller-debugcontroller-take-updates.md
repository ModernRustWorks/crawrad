---
id: controller-debugcontroller-take-updates
kind: doc
target_file: src/controller.rs
line_hint: 188
author: 
created: 
title: "DebugController::take_updates"
tags: []
signature: "pub fn take_updates (& self) -> Vec < Update >"
---

Every update that arrived since the last call, in order.

# Examples

```rust
use crawrad_dap::{AdapterText, DapError, DebugController};

let controller = DebugController::start(
    || Err(DapError::Closed),
    Box::new(AdapterText),
    || {},
);
let updates = controller.take_updates();
assert!(updates.is_empty() || !updates.is_empty());
```
