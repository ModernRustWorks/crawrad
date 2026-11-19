---
id: line-marks-linemarkstore-set
kind: doc
target_file: src/line_marks.rs
line_hint: 53
author: 
created: 
title: "LineMarkStore::set"
tags: []
signature: "pub fn set (& mut self , extension : & str , params : SetLineMarksParams)"
---

`editor/setLineMarks` from `extension`: its marks on that file replaced.

# Examples

```rust
use crawrad_extension_api::marks::SetLineMarksParams;
use crawrad_extension_host::line_marks::LineMarkStore;

let mut store = LineMarkStore::default();
store.set("ext1", SetLineMarksParams { file: "lib.rs".into(), marks: Vec::new() });
assert_eq!(store.revision(), 1);
```
