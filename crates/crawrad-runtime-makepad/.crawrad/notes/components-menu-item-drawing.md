---
id: components-menu-item-drawing
kind: doc
target_file: src/components/menu_item.rs
line_hint: 11
author: 
created: 
title: "DRAWING"
tags: []
signature: "pub static DRAWING : Badge = Badge :: for_class (\"MenuItem\")"
---

The drawing of the `MenuItem`.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::menu_item::DRAWING;

assert_eq!(DRAWING.class_name(), "MenuItem");
```
