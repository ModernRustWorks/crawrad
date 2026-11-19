---
id: dataset-columns-datasetcolumns-state
kind: doc
target_file: src/dataset_columns.rs
line_hint: 110
author: 
created: 
title: "DatasetColumns::state"
tags: []
signature: "pub fn state (& self , url : & str , table : & str) -> Option < & ColumnState >"
---

The state of one `(absolute url, table)`.

# Examples

```rust
use crawrad_host::dataset_columns::DatasetColumns;

let cols = DatasetColumns::default();
assert!(cols.state("sqlite://app.db", "items").is_none());
```
