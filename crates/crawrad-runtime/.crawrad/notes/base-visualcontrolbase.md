---
id: base-visualcontrolbase
kind: doc
target_file: src/base.rs
line_hint: 40
author: 
created: 
title: "VisualControlBase"
tags: []
signature: "pub struct VisualControlBase"
---

Control that has a visual and geometric representation on screen.

Conceptual equivalent of `TControl` in the classic RAD IDE. Groups geometry
(`rect`), visibility (`visible`), enablement (`enabled`), caption (`text`),
and the flag indicating whether it participates in canvas drawing (`visual`).

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime::{ComponentBase, VisualControlBase};

let visual = VisualControlBase {
    base: ComponentBase::new("btn1", "Button1", "Button"),
    rect: Rect::new(10.0, 10.0, 80.0, 24.0),
    visible: true,
    enabled: true,
    text: "Click".to_string(),
    visual: true,
    derived_geometry: false,
};
assert_eq!(visual.name, "Button1");
```
