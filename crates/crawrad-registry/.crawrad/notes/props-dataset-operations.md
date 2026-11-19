---
id: props-dataset-operations
kind: doc
target_file: src/props.rs
line_hint: 94
author: 
created: 
title: "DATASET_OPERATIONS"
tags: []
signature: "pub const DATASET_OPERATIONS : & [& str] = & [\"None\" , \"First\" , \"Prior\" , \"Next\" , \"Last\" , \"Insert\" , \"Edit\" , \"Delete\" , \"Post\" , \"Cancel\" , \"Refresh\" ,]"
---

The operations a dataset-aware standard action can perform on its dataset.

# Examples

```rust
use crawrad_registry::DATASET_OPERATIONS;

assert!(DATASET_OPERATIONS.contains(&"Refresh"));
```
