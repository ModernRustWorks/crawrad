---
id: inspector-model-window
kind: doc
target_file: src/inspector_model.rs
line_hint: 666
author: 
created: 
title: "Window"
tags: []
signature: "pub enum Window"
---

The window `inspector.open_editor` shows for a property.

# Examples

```rust
use crawrad_actions::inspector_model::Window;

let win = Window::Items;
assert_eq!(win, Window::Items);
```
