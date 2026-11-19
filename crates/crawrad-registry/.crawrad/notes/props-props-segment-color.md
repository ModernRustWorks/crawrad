---
id: props-props-segment-color
kind: doc
target_file: src/props.rs
line_hint: 2128
author: 
created: 
title: "Props::segment_color"
tags: []
signature: "pub fn segment_color (default : crawrad_ir :: Color) -> PropDesc"
---

Background color of chevron segments in a `Breadcrumb` (`SegmentColor`, #500).

# Examples

```rust
use crawrad_ir::Color;
use crawrad_registry::Props;

let p = Props::segment_color(Color::rgb(255, 255, 255));
assert_eq!(p.key, "segment_color");
```
