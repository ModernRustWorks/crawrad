---
id: run-tool-no-project
kind: doc
target_file: src/run_tool.rs
line_hint: 17
author: 
created: 
title: "no_project"
tags: []
signature: "pub fn no_project () -> Answer"
---

The answer when no project is open.

# Examples

```rust
use crawrad_mcp::run_tool::no_project;

let (result, outcome) = no_project();
assert_eq!(outcome, "error");
assert!(result.is_error);
```
