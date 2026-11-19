---
id: image-list-model-gallery
kind: doc
target_file: src/image_list_model.rs
line_hint: 197
author: 
created: 
title: "gallery"
tags: []
signature: "pub fn gallery (user_icons : & [GalleryIcon]) -> Vec < GalleryIcon >"
---

Returns the entire gallery with built-ins followed by user icons.

The entire gallery: built-in first, user second.

The order is not taste: what came in the binary is what the person has already seen, and what
they added is what they are looking for — at the end, where they left it.

# Examples

```rust
use crawrad_designer::image_list_model::gallery;

let all = gallery(&[]);
assert!(!all.is_empty());
```
