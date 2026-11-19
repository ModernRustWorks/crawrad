---
id: adapter-dataset-datasetadapter-post
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 180
author: 
created: 
title: "DatasetAdapter::post"
tags: []
signature: "pub fn post (& self) -> Result < () , String >"
---

Commit the current edits or insert into the dataset.

Refused while a bound cell holds text the store would not take: posting
then would write the OLD value and report success, and the person who
clicked Post would never learn that their change was dropped.

# Examples

```rust
use crawrad_binding::{DatasetAdapter, DatasetState};

let adapter = DatasetAdapter::new(&["id"], Vec::new());
adapter.insert().unwrap();
assert!(adapter.post().is_ok());
assert_eq!(adapter.state().get(), DatasetState::Browse);
```

# Errors

Returns an error if any field was refused or if the underlying store fails to post.

# Panics

Panics if the refused fields mutex is poisoned.
