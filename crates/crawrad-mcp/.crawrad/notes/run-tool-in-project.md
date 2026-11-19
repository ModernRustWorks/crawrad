---
id: run-tool-in-project
kind: doc
target_file: src/run_tool.rs
line_hint: 23
author: 
created: 
title: "in_project"
tags: []
signature: "pub fn in_project (root : Option < PathBuf > , run : impl FnOnce (& Path) -> Ran) -> Answer"
---

A cargo run in the project folder, or `no-project` when there is none.

# Examples

```rust
use crawrad_mcp::run_tool::in_project;
use serde_json::json;

let (result, outcome) = in_project(None, |_| Ok((json!({}), "ok".to_string(), true)));
assert_eq!(outcome, "error");
```
