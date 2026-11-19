---
id: image-list-model-imagelisteditorstate-has-selection
kind: doc
target_file: src/image_list_model.rs
line_hint: 332
author: 
created: 
title: "ImageListEditorState::has_selection"
tags: []
signature: "pub fn has_selection (& self) -> bool"
---

Returns whether an image is chosen and exists.

Is there an image chosen, and does it exist?

This is what enables `Replace…` and `Delete`. It was an `is_some() &&
unwrap() < len()` written in the middle of drawing, and written once per
shell would be two.

# Examples

```rust
use crawrad_designer::image_list_model::ImageListEditorState;
use crawrad_ir::Node;

let state = ImageListEditorState::from_node("iml", &Node::new("iml", "ImageList"));
assert!(!state.has_selection());
```
