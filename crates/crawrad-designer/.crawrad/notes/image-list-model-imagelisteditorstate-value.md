---
id: image-list-model-imagelisteditorstate-value
kind: doc
target_file: src/image_list_model.rs
line_hint: 362
author: 
created: 
title: "ImageListEditorState::value"
tags: []
signature: "pub fn value (& self) -> Value"
---

The list ready to go to the document, in `images_data` format.

# Examples

```rust
use crawrad_designer::image_list_model::ImageListEditorState;
use crawrad_ir::{Node, Value};

let state = ImageListEditorState::from_node("iml", &Node::new("iml", "ImageList"));
assert_eq!(state.value(), Value::List(Vec::new()));
```
