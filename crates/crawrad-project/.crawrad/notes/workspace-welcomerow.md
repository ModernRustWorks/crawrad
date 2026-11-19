---
id: workspace-welcomerow
kind: doc
target_file: src/workspace.rs
line_hint: 336
author: 
created: 
title: "WelcomeRow"
tags: []
signature: "pub struct WelcomeRow"
---

One row of the Welcome page's Recent list.

One row of the Welcome page's Recent list (#555), already in words: the
page draws it and decides nothing.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::workspace::WelcomeRow;

let row = WelcomeRow {
    folder: PathBuf::from("proj"),
    name: "proj".into(),
    place: "workspace".into(),
    forms: "1 form".into(),
    age: "today".into(),
};
assert_eq!(row.name, "proj");
```
