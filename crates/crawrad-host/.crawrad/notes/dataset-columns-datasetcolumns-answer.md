---
id: dataset-columns-datasetcolumns-answer
kind: doc
target_file: src/dataset_columns.rs
line_hint: 128
author: 
created: 
title: "DatasetColumns::answer"
tags: []
signature: "pub fn answer (& self , doc : & Document , project_dir : Option < & Path > , dataset_id : & str ,) -> Option < Result < Vec < String > , String > >"
---

The columns of `dataset_id`, or why the probe could not read them.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::dataset_columns::DatasetColumns;

let cols = DatasetColumns::default();
let doc = Document::new("form", "Form");
assert!(cols.answer(&doc, None, "dataset1").is_none());
```
