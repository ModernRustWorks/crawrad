---
id: inspector-model-editor-propeditor-template
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 140
author: 
created: 
title: "PropEditor::template"
tags: []
signature: "fn template (& self) -> & 'static str"
---

The name of the DSL template that draws the row.

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
assert_eq!(editor.template(), "dummy");
```
