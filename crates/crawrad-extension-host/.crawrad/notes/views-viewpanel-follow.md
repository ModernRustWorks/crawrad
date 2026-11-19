---
id: views-viewpanel-follow
kind: doc
target_file: src/views.rs
line_hint: 196
author: 
created: 
title: "ViewPanel::follow"
tags: []
signature: "pub fn follow (& mut self , store : & ViewStore) -> bool"
---

Updates the panel views from the store.

The views as the store holds them now; `true` when they changed since
the last look, and the list has to be drawn again.

# Examples

```rust
use crawrad_extension_host::views::{ViewPanel, ViewStore};

let mut panel = ViewPanel::default();
let store = ViewStore::default();
assert!(panel.follow(&store));
assert!(!panel.follow(&store));
```
