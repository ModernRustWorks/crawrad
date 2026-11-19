---
id: line-marks-linemarkstore
kind: doc
target_file: src/line_marks.rs
line_hint: 12
author: 
created: 
title: "LineMarkStore"
tags: []
signature: "pub struct LineMarkStore"
---

Every extension's marks, kept as each one last sent them.

# Examples

```rust
use crawrad_extension_host::line_marks::LineMarkStore;

let store = LineMarkStore::default();
assert_eq!(store.revision(), 0);
```
