---
id: project-tool-created
kind: doc
target_file: src/project_tool.rs
line_hint: 136
author: 
created: 
title: "created"
tags: []
signature: "pub fn created (made : & Result < PathBuf , String > , name : & str , template : & str) -> Answer"
---

Formats the tool call result for a project creation attempt.

`project.create`'s answer: where the project `name` was made from
`template`, or why it was not.

# Examples

```rust
use crawrad_mcp::project_tool::created;
use std::path::PathBuf;

let res = Ok(PathBuf::from("/tmp/demo"));
let (result, status) = created(&res, "demo", "gui_app");
assert_eq!(status, "applied");
```
