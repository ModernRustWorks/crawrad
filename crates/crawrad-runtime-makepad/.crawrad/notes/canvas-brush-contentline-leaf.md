---
id: canvas-brush-contentline-leaf
kind: doc
target_file: src/canvas/brush.rs
line_hint: 44
author: 
created: 
title: "ContentLine::leaf"
tags: []
signature: "pub fn leaf (level : usize , text : impl Into < String >) -> Self"
---

A line with no children — a `ListBox` item or a `Memo` line.

# Examples

```rust
use crawrad_runtime_makepad::canvas::ContentLine;

let line = ContentLine::leaf(1, "Child");
assert_eq!(line.level, 1);
assert!(!line.branch);
```
