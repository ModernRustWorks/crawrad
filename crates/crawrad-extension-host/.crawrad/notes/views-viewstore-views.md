---
id: views-viewstore-views
kind: doc
target_file: src/views.rs
line_hint: 83
author: 
created: 
title: "ViewStore::views"
tags: []
signature: "pub fn views (& self) -> Vec < ExtensionView >"
---

Every declared view with its tree (empty until the extension sends one).

# Examples

```rust
use crawrad_extension_host::views::ViewStore;

let store = ViewStore::default();
assert!(store.views().is_empty());
```
