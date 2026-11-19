---
id: generator-project-target
kind: doc
target_file: src/generator.rs
line_hint: 139
author: 
created: 
title: "project_target"
tags: []
signature: "pub fn project_target (member : & std :: path :: Path) -> Option < crawrad_codegen :: AlvoGerador >"
---

The framework of the project in `member` (the folder of its `Cargo.toml`).

# Examples

```rust
use std::path::Path;
use crawrad_project::generator::project_target;

assert_eq!(project_target(Path::new("missing_folder")), None);
```
