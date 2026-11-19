---
id: dataset-columns-datasetcolumns-pump
kind: doc
target_file: src/dataset_columns.rs
line_hint: 160
author: 
created: 
title: "DatasetColumns::pump"
tags: []
signature: "pub fn pump (& mut self) -> bool"
---

Takes in what the probes answered; `true` when something changed.

# Examples

```rust
use crawrad_host::dataset_columns::DatasetColumns;

let mut cols = DatasetColumns::default();
assert!(!cols.pump());
```
