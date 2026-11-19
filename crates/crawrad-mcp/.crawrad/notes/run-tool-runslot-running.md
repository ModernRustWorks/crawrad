---
id: run-tool-runslot-running
kind: doc
target_file: src/run_tool.rs
line_hint: 74
author: 
created: 
title: "RunSlot::running"
tags: []
signature: "pub fn running (& mut self) -> Option < u32 >"
---

The kept program's PID while it has not exited.

# Examples

```rust
use crawrad_mcp::run_tool::RunSlot;

let mut slot = RunSlot::default();
assert_eq!(slot.running(), None);
```
