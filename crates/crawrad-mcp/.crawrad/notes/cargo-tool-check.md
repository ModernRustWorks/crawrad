---
id: cargo-tool-check
kind: doc
target_file: src/cargo_tool.rs
line_hint: 98
author: 
created: 
title: "check"
tags: []
signature: "pub fn check (dir : & Path) -> Ran"
---

`build.check`: `cargo check` with its compiler messages.

# Examples

```rust,no_run
// Spawns cargo: compiled here, never run as a doctest.
use crawrad_mcp::cargo_tool::check;
use std::path::Path;

let res = check(Path::new("."));
assert!(res.is_ok(), "cargo could not start");
```
