---
id: targets-data-binding-boundcontrol
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 8
author: 
created: 
title: "BoundControl"
tags: []
signature: "pub struct BoundControl"
---

A visual scalar control bound to a `Dataset` column.

# Examples

```rust
use crawrad_codegen::targets::data_binding::BoundControl;
use crawrad_ir::binding::BindingMode;

let ctrl = BoundControl {
    node_id: "edit1".to_string(),
    node_name: "edit1".to_string(),
    class: "Edit".to_string(),
    prop: "text".to_string(),
    column: "name".to_string(),
    mode: BindingMode::TwoWay,
    dataset_name: "ds".to_string(),
    engine_name: "engine".to_string(),
};
assert_eq!(ctrl.class, "Edit");
```
