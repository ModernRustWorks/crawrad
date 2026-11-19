---
id: dataset-columns-datasetcolumns
kind: doc
target_file: src/dataset_columns.rs
line_hint: 78
author: 
created: 
title: "DatasetColumns"
tags: []
signature: "pub struct DatasetColumns"
---

The columns of every bound dataset of the open form.

# Examples

```rust
use crawrad_host::dataset_columns::DatasetColumns;

let cols = DatasetColumns::default();
assert!(cols.state("sqlite://app.db", "users").is_none());
```
