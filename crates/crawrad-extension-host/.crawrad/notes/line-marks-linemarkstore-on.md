---
id: line-marks-linemarkstore-on
kind: doc
target_file: src/line_marks.rs
line_hint: 78
author: 
created: 
title: "LineMarkStore::on"
tags: []
signature: "pub fn on (& self , file : & Path) -> Vec < (String , LineMark) >"
---

Every extension's marks on `file`, with the extension that owns each.

# Examples

```rust
use crawrad_extension_host::line_marks::LineMarkStore;
use std::path::Path;

let store = LineMarkStore::default();
assert!(store.on(Path::new("src/main.rs")).is_empty());
```
