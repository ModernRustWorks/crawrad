---
id: list-crawradlistboxref-drawn-rows
kind: doc
target_file: src/list.rs
line_hint: 440
author: 
created: 
title: "CrawradListBoxRef::drawn_rows"
tags: []
signature: "pub fn drawn_rows (& self , cx : & Cx) -> Vec < (String , Rect) >"
---

Returns the text and rectangles of rows drawn on the last frame.

The rows drawn on the last frame: text and rectangle (#604), in the
same coordinates as the list's own area.

A row's area is measured in the draw pass that holds the list — inside a
dialog it came out shifted by the dialog's origin (448, 192 in the New
Project) while the list's area is absolute, and a click aimed at it
landed elsewhere. The first drawn row sits at the list's top (the list
is not scrolled), so that offset is put back.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn check_drawn(cx: &Cx, r: &CrawradListBoxRef) {
    let _rows = r.drawn_rows(cx);
}
```
