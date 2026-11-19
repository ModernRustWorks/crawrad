---
id: image-list-model-gallery-file
kind: doc
target_file: src/image_list_model.rs
line_hint: 194
author: 
created: 
title: "GALLERY_FILE"
tags: []
signature: "pub const GALLERY_FILE : & str = \"icones.json\""
---

The file name of the user gallery, inside the configuration folder.

Same folder as the dock layout and recent folders: it is IDE configuration,
not project configuration. An icon added here serves ALL
forms — which is what "gallery" means.

# Examples

```rust
use crawrad_designer::image_list_model::GALLERY_FILE;

assert_eq!(GALLERY_FILE, "icones.json");
```
