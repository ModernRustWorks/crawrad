---
id: dataset-columns-datasetsource
kind: doc
target_file: src/dataset_columns.rs
line_hint: 22
author: 
created: 
title: "DatasetSource"
tags: []
signature: "pub struct DatasetSource"
---

Where a dataset reads from: its connection's url and its table.

# Examples

```rust
use crawrad_host::dataset_columns::DatasetSource;

let source = DatasetSource { url: "sqlite://app.db".into(), table: "items".into() };
assert_eq!(source.table, "items");
```
