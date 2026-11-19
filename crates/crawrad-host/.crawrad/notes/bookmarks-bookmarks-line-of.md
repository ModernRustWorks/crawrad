---
id: bookmarks-bookmarks-line-of
kind: doc
target_file: src/bookmarks.rs
line_hint: 43
author: 
created: 
title: "Bookmarks::line_of"
tags: []
signature: "pub fn line_of (& self , file : & Path , n : u8) -> Option < usize >"
---

The line bookmark `n` is on in `file`, if it is set.

# Examples

```rust
use std::path::Path;
use crawrad_host::Bookmarks;

let mut bookmarks = Bookmarks::default();
bookmarks.set(Path::new("src/main.rs"), &[(1, 10)]);
assert_eq!(bookmarks.line_of(Path::new("src/main.rs"), 1), Some(10));
```
