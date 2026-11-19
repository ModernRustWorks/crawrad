---
id: dataset-columns-datasetcolumns-ensure
kind: doc
target_file: src/dataset_columns.rs
line_hint: 129
author: 
created: 
title: "DatasetColumns::ensure"
tags: []
signature: "pub fn ensure (& mut self , doc : & Document , revision : u64 , project_dir : Option < & Path >)"
---

Starts a probe for every bound dataset not asked about yet.

A new `revision` forgets the failures, so a corrected path or table is
tried again; the answers that came back stay (the key changes by itself
when the url or the table does).

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::dataset_columns::DatasetColumns;

let mut cols = DatasetColumns::default();
let doc = Document::new("form", "Form");
cols.ensure(&doc, 1, None);
```
