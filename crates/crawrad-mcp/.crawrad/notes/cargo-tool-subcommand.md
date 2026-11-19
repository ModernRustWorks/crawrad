---
id: cargo-tool-subcommand
kind: doc
target_file: src/cargo_tool.rs
line_hint: 10
author: 
created: 
title: "subcommand"
tags: []
signature: "pub fn subcommand (args : & Value) -> & 'static str"
---

The subcommand `args.command` asks for: `build`, `test`, else `check`.

# Examples

```rust
use crawrad_mcp::cargo_tool::subcommand;

assert_eq!(subcommand(&serde_json::json!({"command": "build"})), "build");
```
