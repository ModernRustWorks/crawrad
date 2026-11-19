---
id: views-viewstore
kind: doc
target_file: src/views.rs
line_hint: 21
author: 
created: 
title: "ViewStore"
tags: []
signature: "pub struct ViewStore"
---

Every extension's views: what the manifests declare, and the trees.

# Examples

```rust
use crawrad_extension_host::views::ViewStore;

let store = ViewStore::default();
assert_eq!(store.revision(), 0);
```
