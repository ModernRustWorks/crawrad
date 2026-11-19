---
id: components-base-graphic
kind: doc
target_file: src/components/base.rs
line_hint: 136
author: 
created: 
title: "GRAPHIC"
tags: []
signature: "pub const GRAPHIC : & [& str] = & [\"Bevel\" , \"Image\" , \"Label\" , \"LinkLabel\" , \"PaintBox\" , \"Shape\" , \"SpeedButton\" , \"Splitter\" ,]"
---

The controls drawn directly on their parent without an own window.

**The controls drawn on their parent** — classic RAD's `TGraphicControl`.
No window, so no focus and no keys: each one gets [`graphic_events`].

# Examples

```rust
use crawrad_registry::GRAPHIC;

assert!(GRAPHIC.contains(&"Label"));
```
