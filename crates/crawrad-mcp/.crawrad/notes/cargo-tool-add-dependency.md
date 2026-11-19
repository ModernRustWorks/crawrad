---
id: cargo-tool-add-dependency
kind: doc
target_file: src/cargo_tool.rs
line_hint: 166
author: 
created: 
title: "add_dependency"
tags: []
signature: "pub fn add_dependency (dir : & Path , crate_name : & str , args : & Value) -> Ran"
---

`project.add_dependency`: `cargo add`, its `features` and `dev`.

# Examples

```rust,no_run
// Spawns cargo: compiled here, never run as a doctest.
use crawrad_mcp::cargo_tool::add_dependency;
use std::path::Path;

let res = add_dependency(Path::new("."), "serde", &serde_json::json!({}));
assert!(res.is_ok(), "cargo could not start");
```
