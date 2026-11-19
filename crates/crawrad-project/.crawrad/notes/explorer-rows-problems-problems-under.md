---
id: explorer-rows-problems-problems-under
kind: doc
target_file: src/explorer_rows/problems.rs
line_hint: 45
author: 
created: 
title: "Problems::under"
tags: []
signature: "pub fn under (map : & ProblemMap , folder : & Path) -> Self"
---

The problems of every file under `folder`.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::problems::{map, Problems};

let m = map(vec![("src/app/main.rs".to_string(), 1, 0)]);
let p = Problems::under(&m, Path::new("src/app"));
assert_eq!(p.errors, 1);
```
