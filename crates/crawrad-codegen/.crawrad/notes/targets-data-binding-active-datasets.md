---
id: targets-data-binding-active-datasets
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 184
author: 
created: 
title: "active_datasets"
tags: []
signature: "pub fn active_datasets (controls : & [BoundControl] , actions : & [DatasetAction] ,) -> Vec < (String , String) >"
---

Collects distinct dataset and engine pairs from controls and actions.

Collects distinct `(dataset_name, engine_name)` pairs from bound controls
and dataset actions.

# Examples

```rust
use crawrad_codegen::targets::data_binding::active_datasets;

let datasets = active_datasets(&[], &[]);
assert!(datasets.is_empty());
```
