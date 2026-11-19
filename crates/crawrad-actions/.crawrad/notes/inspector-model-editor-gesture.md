---
id: inspector-model-editor-gesture
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 100
author: 
created: 
title: "Gesture"
tags: []
signature: "pub enum Gesture < 'a >"
---

What the user just did to one slot of a row.

# Examples

```rust
use crawrad_actions::inspector_model::editor::Gesture;

let g = Gesture::Toggled(true);
assert!(matches!(g, Gesture::Toggled(true)));
```
