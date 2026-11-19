---
id: cargo-tool-dependencies
kind: doc
target_file: src/cargo_tool.rs
line_hint: 148
author: 
created: 
title: "dependencies"
tags: []
signature: "pub fn dependencies (toml : & str) -> std :: collections :: HashMap < String , String >"
---

Parses raw dependency lines from Cargo.toml content.

`project.get_dependencies`: the `[dependencies]` of a `Cargo.toml`, as
written (name = the rest of the line).

# Examples

```rust
use crawrad_mcp::cargo_tool::dependencies;

let toml = "[dependencies]\nserde = \"1.0\"\n";
let deps = dependencies(toml);
assert_eq!(deps.get("serde").map(|s| s.as_str()), Some("\"1.0\""));
```
