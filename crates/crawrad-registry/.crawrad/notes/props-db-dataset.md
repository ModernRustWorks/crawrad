---
id: props-db-dataset
kind: doc
target_file: src/props.rs
line_hint: 85
author: 
created: 
title: "DB_DATASET"
tags: []
signature: "pub const DB_DATASET : & str = \"DbDataset\""
---

The capability a dataset answers to; a Grid bound to one hides TableName.

# Examples

```rust
use crawrad_registry::props::DB_DATASET;

assert_eq!(DB_DATASET, "DbDataset");
```
