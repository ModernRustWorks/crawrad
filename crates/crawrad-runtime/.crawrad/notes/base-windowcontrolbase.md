---
id: base-windowcontrolbase
kind: doc
target_file: src/base.rs
line_hint: 117
author: 
created: 
title: "WindowControlBase"
tags: []
signature: "pub struct WindowControlBase"
---

Control with hierarchical window and container relationships.

Control with a window/container hierarchical relationship (parent, clip, and tab order).

Conceptual equivalent of `TWinControl` in the classic RAD IDE. Groups visual
hierarchy properties: `parent` in the document, `clip` bounded by the parent
container, and focus ordering `tab_order`.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime::{ComponentBase, VisualControlBase, WindowControlBase};

let win = WindowControlBase {
    visual_base: VisualControlBase {
        base: ComponentBase::new("btn1", "Button1", "Button"),
        rect: Rect::new(10.0, 10.0, 80.0, 24.0),
        visible: true,
        enabled: true,
        text: "Click".to_string(),
        visual: true,
        derived_geometry: false,
    },
    parent: "form1".to_string(),
    children: Vec::new(),
    clip: None,
    tab_order: 0,
};
assert_eq!(win.tab_order, 0);
```
