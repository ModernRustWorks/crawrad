---
id: targets-makepad-components-edit-text-top
kind: doc
target_file: src/targets/makepad/components/edit.rs
line_hint: 217
author: 
created: 
title: "text_top"
tags: []
signature: "pub fn text_top (node : & Node , h : f32) -> f32"
---

Top padding placing single-line capitals in the middle of a field.

The top padding that puts a single line's capitals in the middle of an
`h`-tall field (#574) — shared with the SpinEdit's field.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::edit::text_top;
use crawrad_ir::Node;

let node = Node::new("ed", "Edit");
let top = text_top(&node, 24.0);
assert!(top >= 0.0);
```
