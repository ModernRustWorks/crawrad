---
id: props-props-file-name
kind: doc
target_file: src/props.rs
line_hint: 1675
author: 
created: 
title: "Props::file_name"
tags: []
signature: "pub fn file_name (default : impl Into < String >) -> PropDesc"
---

Selected file name or path (`FileName`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::file_name("doc.txt");
assert_eq!(p.key, "file_name");
```
