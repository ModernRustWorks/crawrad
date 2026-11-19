---
id: explorer-model-name-of
kind: doc
target_file: src/explorer_model.rs
line_hint: 305
author: 
created: 
title: "name_of"
tags: []
signature: "pub fn name_of (path : & Path) -> & str"
---

File name, or the full path when it does not have one.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_model::name_of;

assert_eq!(name_of(Path::new("src/main.rs")), "main.rs");
```
