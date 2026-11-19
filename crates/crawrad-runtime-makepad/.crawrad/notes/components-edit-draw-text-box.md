---
id: components-edit-draw-text-box
kind: doc
target_file: src/components/edit.rs
line_hint: 31
author: 
created: 
title: "draw_text_box"
tags: []
signature: "pub fn draw_text_box (p : & mut Brush , c : & MakepadCtl , r : Rect , text : & str)"
---

Draws a single-line text box of the Edit family.

A single-line text box of the `Edit` family showing `text`: the sunken
box, then the text (masked by its `PasswordChar`) or, when empty, its
`TextHint` — a `MaskEdit` hands its masked text (#71).

# Examples

```rust,no_run
// MakepadCtl and Brush require the Makepad canvas runtime.
use makepad_widgets::Rect;
use crawrad_runtime_makepad::MakepadCtl;
use crawrad_runtime_makepad::canvas::Brush;
use crawrad_runtime_makepad::components::edit::draw_text_box;

fn example(brush: &mut Brush, ctl: &MakepadCtl, rect: Rect) {
    draw_text_box(brush, ctl, rect, "Hello");
}
```
