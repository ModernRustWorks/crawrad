---
id: canvas-atlas-pictureatlas-upload
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 176
author: 
created: 
title: "PictureAtlas::upload"
tags: []
signature: "pub fn upload (& mut self , cx : & mut Cx) -> bool"
---

Pushes the CPU buffer to the GPU texture if dirty.

Returns true when an upload occurred.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::canvas::atlas::PictureAtlas;

fn example(cx: &mut Cx, atlas: &mut PictureAtlas) {
    let _uploaded = atlas.upload(cx);
}
```
