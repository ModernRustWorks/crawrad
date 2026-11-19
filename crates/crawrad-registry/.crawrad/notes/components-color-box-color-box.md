---
id: components-color-box-color-box
kind: doc
target_file: src/components/color_box.rs
line_hint: 4
author: 
created: 
title: "color_box"
tags: []
signature: "pub fn color_box () -> ComponentDesc"
---

A drop-down list of standard colors with preview swatches.

`ColorBox` (#500, wave 1): a drop-down of the standard colours, each with
its swatch (`crawrad_ir::Color::STANDARD`); `Selected` is the chosen colour.

# Examples

```rust
use crawrad_registry::color_box;

let desc = color_box();
assert_eq!(desc.class, "ColorBox");
```
