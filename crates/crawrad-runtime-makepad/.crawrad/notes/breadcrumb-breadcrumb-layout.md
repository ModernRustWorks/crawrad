---
id: breadcrumb-breadcrumb-layout
kind: doc
target_file: src/breadcrumb.rs
line_hint: 65
author: 
created: 
title: "breadcrumb_layout"
tags: []
signature: "pub fn breadcrumb_layout (widths : & [f32] , separator_width : f32 , gap : f32) -> Vec < f32 >"
---

Pure function computing horizontal `x` offsets for each breadcrumb segment.

Between segment `i` and segment `i + 1`, a separator of width `separator_width`
is placed with `gap` spacing on both sides.

# Examples

```rust
use crawrad_runtime_makepad::breadcrumb::breadcrumb_layout;

let widths = [40.0, 50.0, 30.0];
let positions = breadcrumb_layout(&widths, 8.0, 6.0);
assert_eq!(positions, vec![0.0, 60.0, 130.0]);
```
