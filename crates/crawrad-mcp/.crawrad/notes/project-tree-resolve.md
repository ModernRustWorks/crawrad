---
id: project-tree-resolve
kind: doc
target_file: src/project_tree.rs
line_hint: 43
author: 
created: 
title: "resolve"
tags: []
signature: "pub fn resolve (path : & str , root : Option < & Path >) -> std :: path :: PathBuf"
---

Resolves a tool path against an optional project root directory.

A path a tool names: absolute as is, else under the project root (when
there is one).

# Examples

```rust
use crawrad_mcp::project_tree::resolve;
use std::path::Path;

let resolved = resolve("src/lib.rs", Some(Path::new("/root")));
assert_eq!(resolved, Path::new("/root/src/lib.rs"));
```
