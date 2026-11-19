---
id: adapter-dataset-datasetadapter-field-or-create
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 156
author: 
created: 
title: "DatasetAdapter::field_or_create"
tags: []
signature: "pub fn field_or_create (& self , name : & str) -> Observable < String >"
---

Retrieve or dynamically register an observable cell for the specified field.

# Examples

```rust
use crawrad_binding::DatasetAdapter;

let adapter = DatasetAdapter::new(&["name"], Vec::new());
let field = adapter.field_or_create("name");
assert_eq!(field.get(), "");
```

# Panics

Panics if the internal fields mutex is poisoned.
