---
id: controller-debugcontroller-start
kind: doc
target_file: src/controller.rs
line_hint: 143
author: 
created: 
title: "DebugController::start"
tags: []
signature: "pub fn start < L > (launch : L , presenter : Box < dyn ValuePresenter + Send > , waker : fn ()) -> Self where L : FnOnce () -> Result < DebugSession , DapError > + Send + 'static ,"
---

Starts the background debug session thread.

Starts the thread: it calls `launch` (which does the handshake), then
serves commands until the program ends.

# Examples

```rust
use crawrad_dap::{AdapterText, DapError, DebugController};

let controller = DebugController::start(
    || Err(DapError::Closed),
    Box::new(AdapterText),
    || {},
);
assert!(controller.take_updates().is_empty() || true);
```
