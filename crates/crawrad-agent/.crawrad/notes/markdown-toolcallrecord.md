---
id: markdown-toolcallrecord
kind: doc
target_file: src/markdown.rs
line_hint: 39
author: 
created: 
title: "ToolCallRecord"
tags: []
signature: "pub struct ToolCallRecord"
---

Structured record of a tool call.

# Examples

```rust
use crawrad_agent::ToolCallRecord;

let rec = ToolCallRecord::new("read_file", "{}");
assert_eq!(rec.name, "read_file");
```
