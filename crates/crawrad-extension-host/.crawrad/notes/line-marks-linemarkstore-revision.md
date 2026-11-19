---
id: line-marks-linemarkstore-revision
kind: doc
target_file: src/line_marks.rs
line_hint: 73
author: 
created: 
title: "LineMarkStore::revision"
tags: []
signature: "pub fn revision (& self) -> u64"
---

Changes whenever any mark does.

# Examples

```rust
use crawrad_extension_host::line_marks::LineMarkStore;

let store = LineMarkStore::default();
assert_eq!(store.revision(), 0);
```
