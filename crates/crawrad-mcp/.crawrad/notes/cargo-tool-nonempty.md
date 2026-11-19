---
id: cargo-tool-nonempty
kind: doc
target_file: src/cargo_tool.rs
line_hint: 59
author: 
created: 
title: "nonempty"
tags: []
signature: "pub fn nonempty < 'a > (args : & 'a Value , key : & str) -> Option < & 'a str >"
---

A text parameter present and not blank, trimmed.

# Examples

```rust
use crawrad_mcp::cargo_tool::nonempty;

let args = serde_json::json!({"key": "  hello  "});
assert_eq!(nonempty(&args, "key"), Some("hello"));
```
