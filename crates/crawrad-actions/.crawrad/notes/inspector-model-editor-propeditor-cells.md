---
id: inspector-model-editor-propeditor-cells
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 143
author: 
created: 
title: "PropEditor::cells"
tags: []
signature: "fn cells (& self) -> Vec < Cell >"
---

What goes in each named widget of that template.

# Examples

```rust
use crawrad_actions::inspector_model::editor::{Cell, PropEditor};

#[derive(Debug)]
struct DummyEditor;
impl PropEditor for DummyEditor {
    fn template(&self) -> &'static str { "dummy" }
    fn cells(&self) -> Vec<Cell> { Vec::new() }
}

let editor = DummyEditor;
assert!(editor.cells().is_empty());
```
