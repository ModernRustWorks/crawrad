---
id: inspector-model-editors-cursor-cursoreditor
kind: doc
target_file: src/inspector_model/editors/cursor.rs
line_hint: 29
author: 
created: 
title: "CursorEditor"
tags: []
signature: "pub struct CursorEditor"
---

A drop-down over cursor options.

# Examples

```rust
use crawrad_actions::inspector_model::editors::cursor::CursorEditor;
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_ir::Cursor;

let ed = CursorEditor::new(Cursor::Default);
assert_eq!(ed.template(), "RowChoice");
```
