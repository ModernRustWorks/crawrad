---
id: targets-data-binding-egui-after-components
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 371
author: 
created: 
title: "egui_after_components"
tags: []
signature: "pub fn egui_after_components (controls : & [BoundControl]) -> String"
---

Generates egui epilogue code for widget to model writeback.

Egui epilogue after drawing components: widget -> model writeback for
TwoWay / OneWayToSource.

# Examples

```rust
use crawrad_codegen::targets::data_binding::egui_after_components;

let code = egui_after_components(&[]);
assert!(code.is_empty());
```
