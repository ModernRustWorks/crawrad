---
id: recover-read
kind: doc
target_file: src/recover.rs
line_hint: 159
author: 
created: 
title: "read"
tags: []
signature: "pub fn read (root : & Path , file : & Path) -> Option < String >"
---

The text of the copy of `file` (relative to the project).

# Examples

```rust
use std::path::Path;
use crawrad_project::recover::read;

assert_eq!(read(Path::new("."), Path::new("non_existent.rs")), None);
```
