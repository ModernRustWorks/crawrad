---
id: adapter-store-recordstore-len
kind: doc
target_file: src/adapter/store.rs
line_hint: 19
author: 
created: 
title: "RecordStore::len"
tags: []
signature: "fn len (& self) -> usize"
---

Total number of records.

# Examples

```rust
use crawrad_binding::{MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
assert_eq!(store.len(), 0);
```
