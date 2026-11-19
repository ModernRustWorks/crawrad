---
id: canvas-brush-contentline-branch-expanded
kind: doc
target_file: src/canvas/brush.rs
line_hint: 64
author: 
created: 
title: "ContentLine::branch_expanded"
tags: []
signature: "pub fn branch_expanded (level : usize , text : impl Into < String > , expanded : bool) -> Self"
---

A branch with an explicit expanded state.

# Examples

```rust
use crawrad_runtime_makepad::canvas::ContentLine;

let line = ContentLine::branch_expanded(0, "Node", false);
assert!(line.branch);
assert!(!line.expanded);
```
