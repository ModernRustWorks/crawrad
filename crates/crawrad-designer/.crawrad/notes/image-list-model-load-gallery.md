---
id: image-list-model-load-gallery
kind: doc
target_file: src/image_list_model.rs
line_hint: 222
author: 
created: 
title: "load_gallery"
tags: []
signature: "pub fn load_gallery (path : & std :: path :: Path) -> Vec < GalleryIcon >"
---

Reads the user gallery from the file, or returns empty.

**A missing file and an unreadable file amount to the same thing**, and deliberately so:
the built-in gallery remains intact, and the IDE opens. Complaining about broken JSON
in a window that the person opened to choose an icon would replace
their work with ours.

# Examples

```rust
use std::path::Path;
use crawrad_designer::image_list_model::load_gallery;

let icons = load_gallery(Path::new("nonexistent.json"));
assert!(icons.is_empty());
```
