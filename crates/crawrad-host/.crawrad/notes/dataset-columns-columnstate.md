---
id: dataset-columns-columnstate
kind: doc
target_file: src/dataset_columns.rs
line_hint: 64
author: 
created: 
title: "ColumnState"
tags: []
signature: "pub enum ColumnState"
---

What the cache knows about one `(url, table)`.

# Examples

```rust
use crawrad_host::dataset_columns::ColumnState;

let state = ColumnState::Loading;
assert_eq!(state, ColumnState::Loading);
```
