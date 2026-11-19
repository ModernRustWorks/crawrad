---
id: project-tool-get-dependencies
kind: doc
target_file: src/project_tool.rs
line_hint: 97
author: 
created: 
title: "get_dependencies"
tags: []
signature: "pub fn get_dependencies (root : Option < PathBuf >) -> Answer"
---

`project.get_dependencies`: the `[dependencies]` of the project's manifest.

# Examples

```rust
use crawrad_mcp::project_tool::get_dependencies;

let (result, status) = get_dependencies(None);
assert_eq!(status, "error");
```
