---
id: inspector-model-editors-text-text
kind: doc
target_file: src/inspector_model/editors/text.rs
line_hint: 20
author: 
created: 
title: "Text"
tags: []
signature: "pub struct Text"
---

A one-line text field.

# Examples

```rust
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_actions::inspector_model::editors::text::Text;

let editor = Text::new("Button1");
assert_eq!(editor.template(), "RowText");
```
