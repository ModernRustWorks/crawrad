---
id: props-props-dataset-operation
kind: doc
target_file: src/props.rs
line_hint: 481
author: 
created: 
title: "Props::dataset_operation"
tags: []
signature: "pub fn dataset_operation (default : & 'static str) -> PropDesc"
---

Dataset operation performed by an action (`DatasetOperation`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::dataset_operation("Refresh");
assert_eq!(p.key, "dataset_operation");
```
