---
id: dataset-columns-dataset-source
kind: doc
target_file: src/dataset_columns.rs
line_hint: 31
author: 
created: 
title: "dataset_source"
tags: []
signature: "pub fn dataset_source (doc : & Document , dataset_id : & str) -> Option < DatasetSource >"
---

Finds the data source of a bound dataset node in the document.

Follows exactly what the code generator follows
(`crawrad_codegen::targets::shared::bound_dataset_connection`): the `Dataset`
node, its `connection` reference, the `Connection` it names and that one's
`url`; the table is the Dataset's `table`. Any missing link is `None`, and so
is a Dataset that reads through `sql` only.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::dataset_columns::dataset_source;

let doc = Document::new("form", "Form");
assert!(dataset_source(&doc, "missing").is_none());
```
