---
id: adapter-portadb-bridge-portadbbridge-from-dataset
kind: doc
target_file: src/adapter/portadb_bridge.rs
line_hint: 18
author: 
created: 
title: "PortaDbBridge::from_dataset"
tags: []
signature: "pub fn from_dataset (dataset : & Dataset) -> DatasetAdapter"
---

Create a [`DatasetAdapter`] directly connected to a transactional [`Dataset`].

Changes in bound visual controls flow through the underlying [`RecordStore`] directly to the
database engine via [`Dataset::edit`], [`Dataset::set_field`], [`Dataset::post`], and [`Dataset::delete`].

# Examples

```rust,no_run
// Requires an active database engine connection to construct a Dataset.
use portadb::Dataset;
use crawrad_binding::PortaDbBridge;

fn bind(dataset: &Dataset) {
    let adapter = PortaDbBridge::from_dataset(dataset);
    assert_eq!(adapter.collection().len(), 0);
}
```
