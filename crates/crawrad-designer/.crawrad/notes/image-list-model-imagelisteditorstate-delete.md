---
id: image-list-model-imagelisteditorstate-delete
kind: doc
target_file: src/image_list_model.rs
line_hint: 337
author: 
created: 
title: "ImageListEditorState::delete"
tags: []
signature: "pub fn delete (& mut self)"
---

Deletes the chosen image and moves selection to its neighbor.

The selection does NOT disappear: deleting three in sequence must be three clicks in
the same place — the same rule as `delete` in Items Editor.

# Examples

```rust
use crawrad_designer::image_list_model::ImageListEditorState;
use crawrad_ir::Node;

let mut state = ImageListEditorState::from_node("iml", &Node::new("iml", "ImageList"));
state.delete();
assert!(state.entries.is_empty());
```
