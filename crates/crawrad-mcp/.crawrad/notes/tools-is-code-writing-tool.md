---
id: tools-is-code-writing-tool
kind: doc
target_file: src/tools.rs
line_hint: 1088
author: 
created: 
title: "is_code_writing_tool"
tags: []
signature: "pub fn is_code_writing_tool (name : & str) -> bool"
---

Checks whether a tool name performs code modifications.

# Examples

```rust
use crawrad_mcp::tools::is_code_writing_tool;

assert!(is_code_writing_tool("code.edit"));
assert!(!is_code_writing_tool("form.read"));
```
