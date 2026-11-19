---
id: tools-is-save-tool
kind: doc
target_file: src/tools.rs
line_hint: 1205
author: 
created: 
title: "is_save_tool"
tags: []
signature: "pub fn is_save_tool (name : & str) -> bool"
---

Whether a tool name saves a file (refused while a diff review is pending, #182).

# Examples

```rust
use crawrad_mcp::tools::is_save_tool;

assert!(is_save_tool("file.save"));
assert!(!is_save_tool("form.read"));
```
