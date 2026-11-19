---
id: project-tree-walk
kind: doc
target_file: src/project_tree.rs
line_hint: 9
author: 
created: 
title: "walk"
tags: []
signature: "pub fn walk (path : & Path , depth : usize , max_depth : usize) -> Vec < Value >"
---

Recursively walks a directory tree down to a maximum depth.

The entries under `path`, sorted, down to `max_depth` (`depth` is the
level of `path` itself, from 1).

# Examples

```rust
use crawrad_mcp::project_tree::walk;
use std::path::Path;

let entries = walk(Path::new("non_existent_directory_tree"), 1, 2);
assert!(entries.is_empty());
```
