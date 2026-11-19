---
id: canvas-brush-contentline-branch
kind: doc
target_file: src/canvas/brush.rs
line_hint: 54
author: 
created: 
title: "ContentLine::branch"
tags: []
signature: "pub fn branch (level : usize , text : impl Into < String >) -> Self"
---

A line that has children, and therefore earns the marker.

# Examples

```rust
use crawrad_runtime_makepad::canvas::ContentLine;

let line = ContentLine::branch(0, "Node");
assert!(line.branch);
assert!(line.expanded);
```
