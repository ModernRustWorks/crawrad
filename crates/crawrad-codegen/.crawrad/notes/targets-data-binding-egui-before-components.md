---
id: targets-data-binding-egui-before-components
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 366
author: 
created: 
title: "egui_before_components"
tags: []
signature: "pub fn egui_before_components (controls : & [BoundControl]) -> String"
---

Generates egui preamble code syncing model to widgets.

Egui preamble before drawing components: opens datasets and applies model
-> widget sync.

# Examples

```rust
use crawrad_codegen::targets::data_binding::egui_before_components;

let code = egui_before_components(&[]);
assert!(code.is_empty());
```
