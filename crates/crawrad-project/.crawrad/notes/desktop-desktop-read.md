---
id: desktop-desktop-read
kind: doc
target_file: src/desktop.rs
line_hint: 138
author: 
created: 
title: "Desktop::read"
tags: []
signature: "pub fn read (root : & Path) -> Option < Self >"
---

The project's desktop, its tabs whose file is gone dropped.

# Examples

```rust
use std::path::Path;
use crawrad_project::desktop::Desktop;

let d = Desktop::read(Path::new("non_existent_folder"));
assert!(d.is_none());
```
