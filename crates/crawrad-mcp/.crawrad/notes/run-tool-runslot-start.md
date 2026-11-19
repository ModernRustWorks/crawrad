---
id: run-tool-runslot-start
kind: doc
target_file: src/run_tool.rs
line_hint: 44
author: 
created: 
title: "RunSlot::start"
tags: []
signature: "pub fn start (& mut self , root : Option < PathBuf > , args : & Value) -> Answer"
---

`run.start`: `cargo run` in the project, unless one still runs.

# Examples

```rust
use crawrad_mcp::run_tool::RunSlot;
use serde_json::json;

let mut slot = RunSlot::default();
let (result, status) = slot.start(None, &json!({}));
assert_eq!(status, "error");
```
