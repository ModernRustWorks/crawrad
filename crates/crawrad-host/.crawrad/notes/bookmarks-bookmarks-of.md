---
id: bookmarks-bookmarks-of
kind: doc
target_file: src/bookmarks.rs
line_hint: 35
author: 
created: 
title: "Bookmarks::of"
tags: []
signature: "pub fn of (& self , file : & Path) -> Vec < (u8 , usize) >"
---

The bookmarks of `file` as `(number, line)`, by number.

# Examples

```rust
use std::path::Path;
use crawrad_host::Bookmarks;

let bookmarks = Bookmarks::default();
assert!(bookmarks.of(Path::new("src/main.rs")).is_empty());
```
