---
id: canvas-brush-contentline
kind: doc
target_file: src/canvas/brush.rs
line_hint: 30
author: 
created: 
title: "ContentLine"
tags: []
signature: "pub struct ContentLine"
---

One line to be drawn inside a control.

It used to be a `(usize, String)` pair. It gained [`ContentLine::branch`]
because the owner asked for the `TreeView` expand marker — *"without it the
tree is an indented list"* — and that is an answer only the component has:
whoever flattened the tree is the one who knows an item has children, and
the fact needed a ride down to the drawing.

# Examples

```rust
use crawrad_runtime_makepad::canvas::ContentLine;

let line = ContentLine::leaf(0, "Item 1");
assert_eq!(line.level, 0);
assert_eq!(line.text, "Item 1");
```
