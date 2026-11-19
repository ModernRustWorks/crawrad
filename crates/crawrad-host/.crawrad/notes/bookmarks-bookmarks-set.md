---
id: bookmarks-bookmarks-set
kind: doc
target_file: src/bookmarks.rs
line_hint: 48
author: 
created: 
title: "Bookmarks::set"
tags: []
signature: "pub fn set (& mut self , file : & Path , bookmarks : & [(u8 , usize)])"
---

Replaces the bookmarks of `file`; only 1 to 9 are numbers.

# Examples

```rust
use std::path::Path;
use crawrad_host::Bookmarks;

let mut bookmarks = Bookmarks::default();
bookmarks.set(Path::new("src/main.rs"), &[(1, 42)]);
assert_eq!(bookmarks.of(Path::new("src/main.rs")), vec![(1, 42)]);
```
