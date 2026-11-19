---
id: permission-is-read-only-tool
kind: doc
target_file: src/permission.rs
line_hint: 373
author: 
created: 
title: "is_read_only_tool"
tags: []
signature: "pub fn is_read_only_tool (tool_name : & str) -> bool"
---

Determines whether a tool is strictly read-only.

Whether a tool is strictly read-only (no side effects).
Used to honour the `auto_approve_reads` policy.

# Examples

```rust
use crawrad_agent::is_read_only_tool;

assert!(is_read_only_tool("fs.read_file"));
assert!(!is_read_only_tool("fs.write_file"));
```
