---
id: dataset-columns-datasetcolumns-with-probe
kind: doc
target_file: src/dataset_columns.rs
line_hint: 95
author: 
created: 
title: "DatasetColumns::with_probe"
tags: []
signature: "pub fn with_probe (probe : fn () -> Option < PathBuf > , run : fn (& Path , & str , & str) -> Answer ,) -> Self"
---

Creates a dataset column cache using custom probe and run functions.

A cache that finds the probe with `probe` and runs it with `run` (the
tests give it ones that answer without a process).

# Examples

```rust
use crawrad_host::dataset_columns::DatasetColumns;

let cols = DatasetColumns::with_probe(|| None, |_exe, _url, _table| Ok(vec![]));
assert!(cols.state("sqlite://app.db", "users").is_none());
```
