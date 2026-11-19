---
id: views-viewstore-set
kind: doc
target_file: src/views.rs
line_hint: 53
author: 
created: 
title: "ViewStore::set"
tags: []
signature: "pub fn set (& mut self , extension : & str , params : SetTreeParams) -> bool"
---

Sets the tree for a declared view.

`view/setTree` from `extension`: the view's tree replaced. A view
the manifest did not declare is refused (`false`): nothing to draw it in.

# Examples

```rust
use crawrad_extension_api::views::SetTreeParams;
use crawrad_extension_host::views::ViewStore;

let mut store = ViewStore::default();
assert!(!store.set("ext1", SetTreeParams { view_id: "v1".into(), items: Vec::new() }));
```
