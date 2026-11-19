---
id: scheme-preview-crawradschemepreviewref-set
kind: doc
target_file: src/scheme_preview.rs
line_hint: 200
author: 
created: 
title: "CrawradSchemePreviewRef::set"
tags: []
signature: "pub fn set (& self , cx : & mut Cx , background : [f32 ; 4] , rows : Vec < PreviewRow >)"
---

Sets the preview background color and code rows, then requests a redraw.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::scheme_preview::CrawradSchemePreviewRef;

fn update(cx: &mut Cx, preview: &CrawradSchemePreviewRef) {
    preview.set(cx, [0.0, 0.0, 0.0, 1.0], Vec::new());
}
```
