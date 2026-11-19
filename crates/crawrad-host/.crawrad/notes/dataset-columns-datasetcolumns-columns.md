---
id: dataset-columns-datasetcolumns-columns
kind: doc
target_file: src/dataset_columns.rs
line_hint: 115
author: 
created: 
title: "DatasetColumns::columns"
tags: []
signature: "pub fn columns (& self , doc : & Document , project_dir : Option < & Path > , dataset_id : & str ,) -> Option < Vec < String > >"
---

The columns the Inspector shows for `dataset_id`, when they are known.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::dataset_columns::DatasetColumns;

let cols = DatasetColumns::default();
let doc = Document::new("form", "Form");
assert!(cols.columns(&doc, None, "dataset1").is_none());
```
