---
id: list-detail-pos
kind: doc
target_file: src/list.rs
line_hint: 307
author: 
created: 
title: "detail_pos"
tags: []
signature: "pub fn detail_pos (row : Rect , detail : & str , font_size : f32) -> DVec2"
---

Computes the start position of a row's right-aligned detail text.

Where a row's detail starts: right-aligned in `row`, vertically centred,
`font_size` points (#604).

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::list::detail_pos;

let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 20.0) };
let pos = detail_pos(r, "123", 8.0);
assert!(pos.x < 100.0);
```
