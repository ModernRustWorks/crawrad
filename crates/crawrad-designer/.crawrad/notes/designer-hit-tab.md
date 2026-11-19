---
id: designer-hit-tab
kind: doc
target_file: src/lib.rs
line_hint: 415
author: 
created: 
title: "Designer::hit_tab"
tags: []
signature: "pub fn hit_tab (& self , doc : & Document , x : f32 , y : f32) -> Option < TabAction >"
---

What a click on the tab strip means.

The strip is not a component: it is part of `PageControl`, and a click
on it selects nothing — it switches the page, or creates one. Without this
`PageControl` would be a single-page container, because there would be
no way to reach the second.

The calculation is `(x - left) / LARGURA_DA_ABA` and nothing else, which is why
the tab has fixed width: drawing does exactly the same calculation in
reverse, so where the tab appears is where the click finds it.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.hit_tab(&doc, 10.0, 10.0).is_none());
```
