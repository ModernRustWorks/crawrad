---
id: image-list-model-save-gallery
kind: doc
target_file: src/image_list_model.rs
line_hint: 230
author: 
created: 
title: "save_gallery"
tags: []
signature: "pub fn save_gallery (path : & std :: path :: Path , all : & [GalleryIcon]) -> Option < String >"
---

Saves the user gallery to disk.

Saves the user gallery. Returns the error message, when there is one.

Only what the user added goes to disk: saving the built-ins along with it
would freeze the binary's list into a file, and the next version of the IDE would bring
new icons that this file would hide.

# Examples

```rust
use std::path::Path;
use crawrad_designer::image_list_model::save_gallery;

let err = save_gallery(Path::new("/nonexistent_dir_xyz/test.json"), &[]);
assert!(err.is_some());
```
