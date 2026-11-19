---
id: views-viewstore-forget
kind: doc
target_file: src/views.rs
line_hint: 68
author: 
created: 
title: "ViewStore::forget"
tags: []
signature: "pub fn forget (& mut self , extension : & str)"
---

The extension is gone: its views and their trees too.

# Examples

```rust
use crawrad_extension_host::views::ViewStore;

let mut store = ViewStore::default();
store.forget("ext1");
assert_eq!(store.revision(), 0);
```
