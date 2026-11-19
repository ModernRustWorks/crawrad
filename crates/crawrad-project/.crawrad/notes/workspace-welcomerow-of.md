---
id: workspace-welcomerow-of
kind: doc
target_file: src/workspace.rs
line_hint: 352
author: 
created: 
title: "WelcomeRow::of"
tags: []
signature: "pub fn of (recent : & Recent , now : std :: time :: SystemTime) -> Self"
---

The row of a described recent folder, its age counted from `now`.

# Examples

```rust
use std::path::PathBuf;
use std::time::SystemTime;
use crawrad_project::workspace::{Recent, WelcomeRow};

let r = Recent {
    path: PathBuf::from("proj"),
    forms: 1,
    modified: None,
};
let row = WelcomeRow::of(&r, SystemTime::now());
assert_eq!(row.forms, "1 form");
```
