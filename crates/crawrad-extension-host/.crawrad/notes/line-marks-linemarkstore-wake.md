---
id: line-marks-linemarkstore-wake
kind: doc
target_file: src/line_marks.rs
line_hint: 39
author: 
created: 
title: "LineMarkStore::wake"
tags: []
signature: "pub fn wake (& self)"
---

Wakes the window without a change of marks.

Wakes the window without a change of marks: an extension's message
(`window/showMessage`) arrived on its reader thread.

# Examples

```rust
use crawrad_extension_host::line_marks::LineMarkStore;

let store = LineMarkStore::default();
store.wake();
```
