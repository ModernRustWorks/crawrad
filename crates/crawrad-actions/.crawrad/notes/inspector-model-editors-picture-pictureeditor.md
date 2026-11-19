---
id: inspector-model-editors-picture-pictureeditor
kind: doc
target_file: src/inspector_model/editors/picture.rs
line_hint: 25
author: 
created: 
title: "PictureEditor"
tags: []
signature: "pub struct PictureEditor"
---

A picture row: its summary, the `…` and the `×`.

# Examples

```rust
use crawrad_actions::inspector_model::editors::picture::PictureEditor;
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_ir::Picture;

let ed = PictureEditor::new("image1".to_string(), "picture", Picture::empty());
assert_eq!(ed.template(), "RowOpen");
```
