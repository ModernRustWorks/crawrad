---
id: adapter-dataset-datarecord
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 60
author: 
created: 
title: "DataRecord"
tags: []
signature: "pub type DataRecord = BTreeMap < String , String >"
---

Map of column names to stringified values representing record data.

Record data representation: map of column names to stringified values.

# Examples

```rust
use crawrad_binding::DataRecord;

let mut record = DataRecord::new();
record.insert("name".to_string(), "Alice".to_string());
assert_eq!(record.get("name").map(|s| s.as_str()), Some("Alice"));
```
