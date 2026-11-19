---
id: inspector-model-editor-editor
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 177
author: 
created: 
title: "Editor"
tags: []
signature: "pub type Editor = Box < dyn PropEditor >"
---

One editor, owned.

`Box<dyn>` and not an `enum`: see the module doc. The cost is one
allocation per row per frame, over a list that is tens of rows long and is
already rebuilt from the catalogue on every draw.

# Examples

```rust
use crawrad_actions::inspector_model::editor::{Cell, Editor, PropEditor};

#[derive(Debug)]
struct DummyEditor;
impl PropEditor for DummyEditor {
    fn template(&self) -> &'static str { "dummy" }
    fn cells(&self) -> Vec<Cell> { Vec::new() }
}

let editor: Editor = Box::new(DummyEditor);
assert_eq!(editor.template(), "dummy");
```
