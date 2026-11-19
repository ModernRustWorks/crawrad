---
id: targets-data-binding-distinct-datasets
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 203
author: 
created: 
title: "distinct_datasets"
tags: []
signature: "pub fn distinct_datasets (controls : & [BoundControl]) -> Vec < (String , String) >"
---

Collects distinct `(dataset_name, engine_name)` pairs from the active controls.

# Examples

```rust
use crawrad_codegen::targets::data_binding::distinct_datasets;

let datasets = distinct_datasets(&[]);
assert!(datasets.is_empty());
```
