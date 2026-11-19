---
id: line-marks-linemarkstore-forget
kind: doc
target_file: src/line_marks.rs
line_hint: 64
author: 
created: 
title: "LineMarkStore::forget"
tags: []
signature: "pub fn forget (& mut self , extension : & str)"
---

The extension is gone (uninstalled, crashed): its marks too.

# Examples

```rust
use crawrad_extension_host::line_marks::LineMarkStore;

let mut store = LineMarkStore::default();
store.forget("ext1");
assert_eq!(store.revision(), 0);
```
