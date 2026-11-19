---
id: props-props-panel-width
kind: doc
target_file: src/props.rs
line_hint: 1596
author: 
created: 
title: "Props::panel_width"
tags: []
signature: "pub fn panel_width (default : i64) -> PropDesc"
---

The width of a status bar panel in points (`Width`, #458).

Toggles display between single simple panel and multiple panels (`SimplePanel`).
The width of a status bar panel, in points (`Width`, #458).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::panel_width(100);
assert_eq!(p.key, "width");
```
