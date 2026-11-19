---
id: image-list-model-imagelisteditorstate-add-from-gallery
kind: doc
target_file: src/image_list_model.rs
line_hint: 349
author: 
created: 
title: "ImageListEditorState::add_from_gallery"
tags: []
signature: "pub fn add_from_gallery (& mut self , icon : & GalleryIcon)"
---

Adds an icon from the GALLERY at the end, and leaves it selected.

There used to be an `add(preset: usize)` here that indexed the BUILT-IN
table. It was removed with the gallery (09/17/2026): table index and gallery
index are different lists, and keeping both was keeping the risk
of a caller using the wrong index — which causes no error, only puts the
wrong icon in ImageList.

# Examples

```rust
use crawrad_designer::image_list_model::{GalleryIcon, ImageListEditorState};
use crawrad_ir::Node;

let mut state = ImageListEditorState::from_node("iml", &Node::new("iml", "ImageList"));
let icon = GalleryIcon {
    name: "Save".to_string(),
    symbol: "💾".to_string(),
    category: "File".to_string(),
    builtin: true,
};
state.add_from_gallery(&icon);
assert_eq!(state.entries.len(), 1);
```
