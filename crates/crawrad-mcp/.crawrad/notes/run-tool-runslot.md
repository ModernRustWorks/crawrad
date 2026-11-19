---
id: run-tool-runslot
kind: doc
target_file: src/run_tool.rs
line_hint: 39
author: 
created: 
title: "RunSlot"
tags: []
signature: "pub struct RunSlot (Option < Child >)"
---

The program `run.start` started, until `run.stop` ends it.

# Examples

```rust
use crawrad_mcp::run_tool::RunSlot;

let slot = RunSlot::default();
```
