---
id: debug-debug-start
kind: doc
target_file: src/debug.rs
line_hint: 85
author: 
created: 
title: "Debug::start"
tags: []
signature: "pub fn start (& mut self , controller : DebugController)"
---

Takes over a started debug controller, ending any older session.

Takes over a started controller; any older session is ended first
(dropping its controller ends its program).

# Examples

```rust,no_run
// Spawns a background thread controller.
use crawrad_host::Debug;

let mut debug = Debug::default();
let controller = crawrad_dap::DebugController::start(
    || Err(crawrad_dap::DapError::Closed),
    Box::new(crawrad_dap::AdapterText),
    || {},
);
debug.start(controller);
assert!(debug.is_active());
```
