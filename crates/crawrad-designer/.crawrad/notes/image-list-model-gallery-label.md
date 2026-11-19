---
id: image-list-model-gallery-label
kind: doc
target_file: src/image_list_model.rs
line_hint: 371
author: 
created: 
title: "gallery_label"
tags: []
signature: "pub fn gallery_label (i : & GalleryIcon) -> String"
---

Formats a gallery row label for an icon.

A row in the **Add Icons from Catalog** window: what is read from each icon.

# Examples

```rust
use crawrad_designer::image_list_model::{gallery_label, GalleryIcon};

let icon = GalleryIcon {
    name: "Save".to_string(),
    symbol: "💾".to_string(),
    category: "File".to_string(),
    builtin: true,
};
assert_eq!(gallery_label(&icon), "💾  Save (File)");
```
