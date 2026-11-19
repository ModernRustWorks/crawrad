---
id: list-draw-details
kind: doc
target_file: src/list.rs
line_hint: 274
author: 
created: 
title: "draw_details"
tags: []
signature: "pub fn draw_details (cx : & mut Cx2d , draw : & mut DrawText , selected_color : Vec4 , font_size : f32 , details : Vec < (Rect , String , bool) > ,)"
---

Draws each row's trailing detail text right-aligned after rendering rows.

Draws each row's detail (row rect, text, selected) at the row's right end,
AFTER every row: a row whose look differs is its own draw call, and a
detail drawn before it would end up under it (#604).

# Examples

```rust,no_run
// Makepad Cx2d and DrawText require the Makepad application runtime.
use makepad_widgets::{vec4, Cx2d, DrawText};
use crawrad_runtime_makepad::list::draw_details;

fn draw(cx: &mut Cx2d, draw: &mut DrawText) {
    draw_details(cx, draw, vec4(1.0, 1.0, 1.0, 1.0), 12.0, Vec::new());
}
```
