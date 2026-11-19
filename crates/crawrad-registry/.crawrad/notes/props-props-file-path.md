---
id: props-props-file-path
kind: doc
target_file: src/props.rs
line_hint: 543
author: 
created: 
title: "Props::file_path"
tags: []
signature: "pub fn file_path (default : & str) -> PropDesc"
---

Path to file on disk (`FilePath`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::file_path("config.json");
assert_eq!(p.key, "file_path");
```
