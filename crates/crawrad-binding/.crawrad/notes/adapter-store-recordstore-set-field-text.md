---
id: adapter-store-recordstore-set-field-text
kind: doc
target_file: src/adapter/store.rs
line_hint: 45
author: 
created: 
title: "RecordStore::set_field_text"
tags: []
signature: "fn set_field_text (& self , name : & str , text : & str) -> Result < () , String >"
---

Updates a buffered field value as text, parsed and validated by the store.

# Examples

```rust
use crawrad_binding::{MemoryStore, RecordStore};

let store = MemoryStore::new(&["id"], Vec::new());
store.insert().unwrap();
assert!(store.set_field_text("id", "1").is_ok());
```

# Errors

Returns an error if the field is not in edit or insert mode, or if validation fails.
