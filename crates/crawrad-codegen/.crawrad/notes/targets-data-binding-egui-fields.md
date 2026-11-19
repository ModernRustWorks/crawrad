---
id: targets-data-binding-egui-fields
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 242
author: 
created: 
title: "egui_fields"
tags: []
signature: "pub fn egui_fields (controls : & [BoundControl]) -> (String , String)"
---

Form struct field declarations and `Default` initializers for the egui target.

# Examples

```rust
use crawrad_codegen::targets::data_binding::egui_fields;

let (fields, defaults) = egui_fields(&[]);
assert!(fields.is_empty());
assert!(defaults.is_empty());
```
