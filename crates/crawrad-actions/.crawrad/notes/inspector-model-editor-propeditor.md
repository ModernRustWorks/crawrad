---
id: inspector-model-editor-propeditor
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 138
author: 
created: 
title: "PropEditor"
tags: []
signature: "pub trait PropEditor : Debug"
---

One editor of one row.

Everything has a default except [`Self::template`] and [`Self::cells`]: an
editor that forgets what a gesture means is read-only and still reads, but
one that forgets its template draws nothing at all.

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
