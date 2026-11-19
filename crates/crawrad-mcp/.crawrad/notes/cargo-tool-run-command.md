---
id: cargo-tool-run-command
kind: doc
target_file: src/cargo_tool.rs
line_hint: 194
author: 
created: 
title: "run_command"
tags: []
signature: "pub fn run_command (dir : & Path , args : & Value) -> std :: process :: Command"
---

`run.start`'s command: `cargo run`, then `--` and `args.args`.

# Examples

```rust
use crawrad_mcp::cargo_tool::run_command;
use std::path::Path;

let cmd = run_command(Path::new("."), &serde_json::json!({}));
assert_eq!(cmd.get_program(), "cargo");
```
