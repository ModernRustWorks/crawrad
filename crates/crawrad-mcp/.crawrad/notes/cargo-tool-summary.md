---
id: cargo-tool-summary
kind: doc
target_file: src/cargo_tool.rs
line_hint: 84
author: 
created: 
title: "summary"
tags: []
signature: "pub fn summary (msg : & Value) -> Value"
---

A compiler message in short: level, text, the first span's place.

# Examples

```rust
use crawrad_mcp::cargo_tool::summary;

let s = summary(&serde_json::json!({"level": "warning", "message": "unused"}));
assert_eq!(s["level"], "warning");
```
