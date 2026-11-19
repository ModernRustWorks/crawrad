---
id: canvas-brush-brush-content
kind: doc
target_file: src/canvas/brush.rs
line_hint: 870
author: 
created: 
title: "Brush::content"
tags: []
signature: "pub fn content (& mut self , area : Rect , ct : & Content)"
---

The lines of a `Memo`, of a `ListBox` or of a `TreeView`.

Where they COME FROM is the component itself
(`crate::componentes::ComponentDrawing::lines`) and how many FIT is
[`lines_that_fit`] — both measured without a window. All that is left
here is painting: the highlight of the chosen line, the marker of whoever
has children, and the text.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Content, ContentLine, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 100.0) };
let lines = [ContentLine::leaf(0, "Item")];
let ct = Content { lines: &lines, line_height: 18.0, color: [0.0, 0.0, 0.0, 1.0], selected: None };
brush.content(r, &ct);
assert!(!probe.marks().is_empty());
```
