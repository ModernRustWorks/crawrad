---
id: line-marks-linemarkstore-set-waker
kind: doc
target_file: src/line_marks.rs
line_hint: 34
author: 
created: 
title: "LineMarkStore::set_waker"
tags: []
signature: "pub fn set_waker (& mut self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when marks change.

# Examples

```rust
use crawrad_extension_host::line_marks::LineMarkStore;

let mut store = LineMarkStore::default();
store.set_waker(|| {});
```
