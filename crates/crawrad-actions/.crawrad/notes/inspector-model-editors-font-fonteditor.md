---
id: inspector-model-editors-font-fonteditor
kind: doc
target_file: src/inspector_model/editors/font.rs
line_hint: 34
author: 
created: 
title: "FontEditor"
tags: []
signature: "pub struct FontEditor"
---

The summary row that folds open into seven sub-rows.

# Examples

```rust
use crawrad_actions::inspector_model::editors::font::FontEditor;
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_ir::Font;

let ed = FontEditor::new("label1".to_string(), "font", Font::default(), false);
assert_eq!(ed.template(), "RowFoldOpen");
```
