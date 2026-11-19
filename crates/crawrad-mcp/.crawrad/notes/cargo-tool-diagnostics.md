---
id: cargo-tool-diagnostics
kind: doc
target_file: src/cargo_tool.rs
line_hint: 138
author: 
created: 
title: "diagnostics"
tags: []
signature: "pub fn diagnostics (dir : & Path) -> Ran"
---

Collects compiler diagnostics from cargo check in short form.

`build.get_diagnostics`: the compiler messages of `cargo check`, in short.
Reading them succeeds even when the build does not.

# Examples

```rust,no_run
// Spawns cargo: compiled here, never run as a doctest.
use crawrad_mcp::cargo_tool::diagnostics;
use std::path::Path;

let res = diagnostics(Path::new("."));
assert!(res.is_ok(), "cargo could not start");
```
