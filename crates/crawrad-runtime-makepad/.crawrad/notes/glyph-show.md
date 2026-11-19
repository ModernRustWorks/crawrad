---
id: glyph-show
kind: doc
target_file: src/glyph.rs
line_hint: 23
author: 
created: 
title: "show"
tags: []
signature: "pub fn show (cx : & mut Cx , ui : & WidgetRef , path : & [LiveId] , button : & WidgetRef , bytes : & [u8] , num_glyphs : i64 , visible : bool , shown : & mut Shown ,)"
---

Displays button glyph image bytes and adjusts button caption padding.

Shows `bytes` in the glyph view at `path` beside `button`, the first of
`num_glyphs`, while `visible`; a picture this target cannot decode, or
none, leaves the caption alone.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{live_id, Cx, LiveId, WidgetRef};
use crawrad_runtime_makepad::glyph::{show, Shown};

fn update_glyph(cx: &mut Cx, ui: &WidgetRef, button: &WidgetRef, shown: &mut Shown) {
    show(cx, ui, &[live_id!(glyph_view)], button, &[], 1, false, shown);
}
```
