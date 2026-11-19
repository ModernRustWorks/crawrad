---
id: adapter-store-recordstore-post
kind: doc
target_file: src/adapter/store.rs
line_hint: 48
author: 
created: 
title: "RecordStore::post"
tags: []
signature: "fn post (& self) -> Result < () , String >"
---

Commits active buffered changes to the underlying store.

Commits the active buffered changes (updates or insertions) to the underlying store.

# Examples

```rust
use crawrad_binding::{DatasetState, MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
store.insert().unwrap();
store.set_field_text("id", "1").unwrap();
assert!(store.post().is_ok());
assert_eq!(store.state(), DatasetState::Browse);
```

# Errors

Returns an error if no edit or insert buffer is active or if persistence fails.
