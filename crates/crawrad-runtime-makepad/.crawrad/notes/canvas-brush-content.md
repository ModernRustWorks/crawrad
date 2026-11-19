---
id: canvas-brush-content
kind: doc
target_file: src/canvas/brush.rs
line_hint: 75
author: 
created: 
title: "Content"
tags: []
signature: "pub struct Content < 'a >"
---

What a control asks to have drawn as content, and by which ruler.

Line heights are in DESIGN units and every component brings its own from the
RXP — the `ListBox` steps by 18 because that is the `interact_size.y` the
live form draws the real line with (`crawrad-runtime-egui/src/lib.rs:3008`), and
a designer that disagreed would show a different item count from the
application (divergence **D3** of the backstage study).

# Examples

```rust
use crawrad_runtime_makepad::canvas::Content;

let content = Content {
    lines: &[],
    line_height: 18.0,
    color: [0.0, 0.0, 0.0, 1.0],
    selected: None,
};
assert_eq!(content.line_height, 18.0);
```
