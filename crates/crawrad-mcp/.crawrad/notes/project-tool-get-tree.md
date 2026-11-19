---
id: project-tool-get-tree
kind: doc
target_file: src/project_tool.rs
line_hint: 23
author: 
created: 
title: "get_tree"
tags: []
signature: "pub fn get_tree (root : Option < PathBuf > , args : & Value) -> Answer"
---

`project.get_tree`: the project's folders and files, `max_depth` deep.

# Examples

```rust
use crawrad_mcp::project_tool::get_tree;
use serde_json::json;

let (result, status) = get_tree(None, &json!({}));
assert_eq!(status, "error");
```
