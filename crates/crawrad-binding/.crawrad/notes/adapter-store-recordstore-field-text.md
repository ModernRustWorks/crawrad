---
id: adapter-store-recordstore-field-text
kind: doc
target_file: src/adapter/store.rs
line_hint: 36
author: 
created: 
title: "RecordStore::field_text"
tags: []
signature: "fn field_text (& self , name : & str) -> Option < String >"
---

Returns the active text representation of a column.

Returns the active text representation of a column (edit buffer while editing, else current row).

# Examples

```rust
use crawrad_binding::{DataRecord, MemoryStore, RecordStore};

let mut row = DataRecord::new();
row.insert("name".to_string(), "Alice".to_string());
let store = MemoryStore::new(&["name"], vec![row]);
assert_eq!(store.field_text("name").as_deref(), Some("Alice"));
```
