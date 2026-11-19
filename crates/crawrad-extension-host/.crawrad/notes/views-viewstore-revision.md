---
id: views-viewstore-revision
kind: doc
target_file: src/views.rs
line_hint: 78
author: 
created: 
title: "ViewStore::revision"
tags: []
signature: "pub fn revision (& self) -> u64"
---

Changes whenever a view or a tree does.

# Examples

```rust
use crawrad_extension_host::views::ViewStore;

let store = ViewStore::default();
assert_eq!(store.revision(), 0);
```
