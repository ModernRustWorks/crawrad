---
id: cargo-tool-build
kind: doc
target_file: src/cargo_tool.rs
line_hint: 115
author: 
created: 
title: "build"
tags: []
signature: "pub fn build (dir : & Path , args : & Value) -> Ran"
---

`build.build`: `cargo build`, `--release` when `args.release`.

# Examples

```rust,no_run
// Spawns cargo: compiled here, never run as a doctest.
use crawrad_mcp::cargo_tool::build;
use std::path::Path;

let res = build(Path::new("."), &serde_json::json!({}));
assert!(res.is_ok(), "cargo could not start");
```
