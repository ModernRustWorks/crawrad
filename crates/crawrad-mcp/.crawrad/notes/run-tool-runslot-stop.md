---
id: run-tool-runslot-stop
kind: doc
target_file: src/run_tool.rs
line_hint: 80
author: 
created: 
title: "RunSlot::stop"
tags: []
signature: "pub fn stop (& mut self) -> Answer"
---

`run.stop`: ends the kept program, if there is one.

# Examples

```rust
use crawrad_mcp::run_tool::RunSlot;

let mut slot = RunSlot::default();
let (result, status) = slot.stop();
assert_eq!(status, "applied");
```
