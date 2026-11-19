---
id: canvas-brush-layer
kind: doc
target_file: src/canvas/brush.rs
line_hint: 152
author: 
created: 
title: "Layer"
tags: []
signature: "pub enum Layer"
---

One canvas painting layer — **one shader, one step**.

# Why this exists, and what it cost to find out

In Makepad the painting order goes by SHADER, not by call order:
`find_appendable_drawcall` appends every instance to the first draw call of
that same shader it finds in the current list
(`makepad-platform-1.0.0/src/draw_list.rs:334-343`), and the `zbias` — the
real Z — is handed out per CALL at render time
(`os/apple/metal.rs:109-111`). What rules, therefore, is **the order in
which the FIRST instance of each shader was emitted**.

And that order, left to chance, is the DOCUMENT order. Measured on the
owner's screen, with two forms side by side:

| first component | what happens |
|---|---|
| `Button` (box before text) | `DrawCaixa` opens first, the text lands on top — **right by accident** |
| `Label` (text with no box) | `DrawText` opens first, **every box covers every text** |

The IDE's `form_main.crawform` starts with a `Label`, and the outcome was
exactly that: empty buttons with **the accelerator underline alone in the
middle** (the stroke is another layer and survived), a missing `ListBox`
item, a missing `Memo` line, a missing `TreeView` node, missing `ComboBox`
text and missing badge initials — everything that is text with something
drawn over it.

The cure is to open the layers **on purpose**, in the order of this stack,
before painting anything (`Brush::open_layers`). From then on no
document and no host can invert a thing.

# Examples

```rust
use crawrad_runtime_makepad::canvas::Layer;

assert_eq!(Layer::Background, Layer::Background);
```
