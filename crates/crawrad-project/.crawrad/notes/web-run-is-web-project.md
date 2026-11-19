---
id: web-run-is-web-project
kind: doc
target_file: src/web_run.rs
line_hint: 13
author: 
created: 
title: "is_web_project"
tags: []
signature: "pub fn is_web_project (root : & Path , package : & str) -> bool"
---

Whether `package` of the workspace at `root` is marked as a browser app.

# Examples

```rust
use std::path::Path;
use crawrad_project::web_run::is_web_project;

assert!(!is_web_project(Path::new("missing"), "demo"));
```
