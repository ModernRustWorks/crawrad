---
id: adapter-store-recordstore-position
kind: doc
target_file: src/adapter/store.rs
line_hint: 27
author: 
created: 
title: "RecordStore::position"
tags: []
signature: "fn position (& self) -> Option < usize >"
---

0-based index of the currently active record, or `None` if empty.

# Examples

```rust
use crawrad_binding::{MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
assert_eq!(store.position(), None);
```
