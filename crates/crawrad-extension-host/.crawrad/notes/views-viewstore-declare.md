---
id: views-viewstore-declare
kind: doc
target_file: src/views.rs
line_hint: 45
author: 
created: 
title: "ViewStore::declare"
tags: []
signature: "pub fn declare (& mut self , extension : & str , views : & [ViewContribution])"
---

The views `extension`'s manifest declares (the previous ones go).

# Examples

```rust
use crawrad_extension_api::views::ViewContribution;
use crawrad_extension_host::views::ViewStore;

let mut store = ViewStore::default();
store.declare("ext1", &[ViewContribution { id: "v1".into(), title: "V1".into(), icon: None }]);
assert_eq!(store.views().len(), 1);
```
