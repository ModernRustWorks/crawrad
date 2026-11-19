---
id: explorer-rows-problems-problems-of
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 40
author: 
created: 
title: "Problems::of"
tags: []
signature: "pub fn of (map : & ProblemMap , file : & Path) -> Self"
---

The problems of `file`.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::problems::{map, Problems};

let m = map(vec![("src/main.rs".to_string(), 2, 1)]);
let p = Problems::of(&m, Path::new("src/main.rs"));
assert_eq!(p.errors, 2);
```
