---
id: targets-data-binding-egui-before-components-all
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 346
author: 
created: 
title: "egui_before_components_all"
tags: []
signature: "pub fn egui_before_components_all (controls : & [BoundControl] , actions : & [DatasetAction]) -> String"
---

Generates egui preamble code before drawing components.

Egui preamble before drawing components: opens datasets, sets dataset
action enabled state, and applies model -> widget sync.

# Examples

```rust
use crawrad_codegen::targets::data_binding::egui_before_components_all;

let code = egui_before_components_all(&[], &[]);
assert!(code.is_empty());
```
