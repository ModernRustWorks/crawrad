---
id: inspector-model-editor-cell
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 55
author: 
created: 
title: "Cell"
tags: []
signature: "pub enum Cell"
---

What an editor puts into one of its slots.

The variants are the **kinds of widget** the templates hold, not the kinds
of value the catalogue has: two different property kinds that both draw a
drop-down produce the same [`Cell::Choice`], and the shell writes both with
the same three lines.

# Examples

```rust
use crawrad_actions::inspector_model::editor::{Cell, Slot};

let cell = Cell::Field { slot: Slot::Value, text: "foo".to_string() };
assert_eq!(cell.slot(), Slot::Value);
```
