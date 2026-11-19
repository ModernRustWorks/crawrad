---
id: targets-data-binding-egui-fields-all
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 208
author: 
created: 
title: "egui_fields_all"
tags: []
signature: "pub fn egui_fields_all (controls : & [BoundControl] , actions : & [DatasetAction]) -> (String , String)"
---

Generates egui struct field declarations and defaults for all datasets.

Form struct field declarations and `Default` initializers for the egui
target, covering datasets used by controls or actions.

# Examples

```rust
use crawrad_codegen::targets::data_binding::egui_fields_all;

let (fields, defaults) = egui_fields_all(&[], &[]);
assert!(fields.is_empty());
assert!(defaults.is_empty());
```
