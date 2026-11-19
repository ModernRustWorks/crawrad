---
id: targets-data-binding-datasetaction
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 29
author: 
created: 
title: "DatasetAction"
tags: []
signature: "pub struct DatasetAction"
---

An `Action` component bound to a dataset operation (ADR 0041, #788).

# Examples

```rust
use crawrad_codegen::targets::data_binding::DatasetAction;

let act = DatasetAction {
    node_id: "act1".to_string(),
    action_name: "act_next".to_string(),
    operation: "Next".to_string(),
    dataset_name: "ds".to_string(),
    engine_name: "engine".to_string(),
    has_on_execute: false,
    has_on_update: false,
};
assert_eq!(act.operation, "Next");
```
