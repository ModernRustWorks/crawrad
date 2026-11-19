---
id: watch-ignored
kind: doc
target_file: src/watch.rs
line_hint: 21
author: 
created: 
title: "ignored"
tags: []
signature: "pub fn ignored (root : & Path , path : & Path) -> bool"
---

Returns whether a path should be ignored under root.

Pure: should this path be ignored? (component `target`, `.git` or `.crawrad` anywhere below `root`)

# Examples

```rust
use std::path::Path;
use crawrad_os::watch::ignored;

assert!(ignored(Path::new("/proj"), Path::new("/proj/target/debug/app")));
assert!(!ignored(Path::new("/proj"), Path::new("/proj/src/main.rs")));
```
