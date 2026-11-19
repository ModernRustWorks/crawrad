---
id: controller-debugcontroller-send
kind: doc
target_file: src/controller.rs
line_hint: 183
author: 
created: 
title: "DebugController::send"
tags: []
signature: "pub fn send (& self , command : Command) -> bool"
---

Queues a command for the debug session thread.

Queues a command. `false` when the session is already over.

# Examples

```rust
use crawrad_dap::{AdapterText, Command, DapError, DebugController};

let controller = DebugController::start(
    || Err(DapError::Closed),
    Box::new(AdapterText),
    || {},
);
let _ = controller.send(Command::Stop);
```
