---
id: process-cargocall-metadata
kind: doc
target_file: src/process.rs
line_hint: 97
author: 
created: 
title: "CargoCall::metadata"
tags: []
signature: "pub fn metadata (dir : impl Into < PathBuf >) -> Self"
---

Ask cargo about the workspace with `--no-deps`.

Ask cargo about the workspace. `--no-deps` because the question is
always about the user's own members, and resolving the dependency graph
turns a fast answer into a slow one.

# Examples

```rust
use crawrad_os::process::CargoCall;

let call = CargoCall::metadata(".");
let cmd = call.command();
assert_eq!(cmd.get_program(), "cargo");
```
