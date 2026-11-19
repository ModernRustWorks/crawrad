---
id: items-editor-selectedfields-image-text
kind: doc
target_file: src/items_editor.rs
line_hint: 312
author: 
created: 
title: "SelectedFields::image_text"
tags: []
signature: "pub fn image_text (& self) -> String"
---

The Image Index field's text: empty when it does not apply.

# Examples

```rust
use crawrad_actions::items_editor::SelectedFields;

let fields = SelectedFields {
    can_add_child: false,
    text: Some("Item".into()),
    expanded: None,
    image_index: Some(3),
};
assert_eq!(fields.image_text(), "3");
```
