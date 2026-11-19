---
id: adapter-store-recordstore-on-change
kind: doc
target_file: src/adapter/store.rs
line_hint: 60
author: 
created: 
title: "RecordStore::on_change"
tags: []
signature: "fn on_change (& self , callback : Box < dyn Fn () + Send + Sync >)"
---

Registers a callback invoked whenever records, cursor, state, or fields mutate.

# Examples

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crawrad_binding::{MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
let called = Arc::new(AtomicBool::new(false));
let called_clone = Arc::clone(&called);
store.on_change(Box::new(move || called_clone.store(true, Ordering::SeqCst)));
store.insert().unwrap();
assert!(called.load(Ordering::SeqCst));
```
