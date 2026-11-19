---
id: canvas-brush-primitives
kind: doc
target_file: src/canvas/brush.rs
line_hint: 226
author: 
created: 
title: "Primitives"
tags: []
signature: "pub struct Primitives < 'a >"
---

The canvas shaders, lent to the [`Brush`] in one block.

Every field is a `#[live]` of [`crate::canvas::DesignCanvas`]: the borrow is
of **disjoint** fields, and that is why the widget can go on reading its own
model while the brush paints.

# Examples

```rust,ignore
// Not reachable from outside the crate: shown, not run.
use crate::canvas::brush::Primitives;

fn check_primitives<'a>(_p: &Primitives<'a>) {}
```
