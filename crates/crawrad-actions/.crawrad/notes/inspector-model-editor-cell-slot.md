---
id: inspector-model-editor-cell-slot
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 87
author: 
created: 
title: "Cell::slot"
tags: []
signature: "pub fn slot (& self) -> Slot"
---

Where it goes.

# Examples

```rust
use crawrad_actions::inspector_model::editor::{Cell, Slot};

let cell = Cell::Field { slot: Slot::Value, text: "abc".to_string() };
assert_eq!(cell.slot(), Slot::Value);
```
