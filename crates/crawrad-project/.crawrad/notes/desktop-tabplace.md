---
id: desktop-tabplace
kind: doc
target_file: src/desktop.rs
line_hint: 38
author: 
created: 
title: "TabPlace"
tags: []
signature: "pub struct TabPlace"
---

Represents one open editor tab and its caret position.

One open tab: its file, relative to the project, and its caret, both
0-based (line, byte), as the editor counts them.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::desktop::TabPlace;

let tab = TabPlace {
    file: PathBuf::from("src/main.rs"),
    line: 1,
    byte: 0,
};
assert_eq!(tab.line, 1);
```
