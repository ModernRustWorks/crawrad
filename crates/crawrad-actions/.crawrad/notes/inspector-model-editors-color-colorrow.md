---
id: inspector-model-editors-color-colorrow
kind: doc
target_file: src/inspector_model/editors/color.rs
line_hint: 30
author: 
created: 
title: "ColorRow"
tags: []
signature: "pub struct ColorRow"
---

A color swatch and text editor with a Color dialog button.

A patch of colour next to the text that spells it, and a button opening the Color dialog.

# Examples

```rust
use crawrad_actions::inspector_model::editors::color::ColorRow;
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_ir::Color;

let row = ColorRow::new("form".to_string(), "color", Color::rgb(0, 0, 0));
assert_eq!(row.template(), "RowColor");
```
