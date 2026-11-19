---
id: tools-code-writing-tool-names
kind: doc
target_file: src/tools.rs
line_hint: 1083
author: 
created: 
title: "code_writing_tool_names"
tags: []
signature: "pub fn code_writing_tool_names () -> Vec < & 'static str >"
---

Returns the list of all code writing tool names from the catalogue.

Any tool in this list MUST pass through the diff review gate.

# Examples

```rust
use crawrad_mcp::tools::code_writing_tool_names;

let names = code_writing_tool_names();
assert!(names.contains(&"code.edit"));
```
