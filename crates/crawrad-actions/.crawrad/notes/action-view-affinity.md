---
id: action-view-affinity
kind: doc
target_file: src/lib.rs
line_hint: 502
author: 
created: 
title: "Action::view_affinity"
tags: []
signature: "pub fn view_affinity (& self) -> ViewAffinity"
---

Returns the view affinity of this action.

# Examples

```rust
use crawrad_actions::by_id;

let action = by_id("file.save").unwrap();
let _affinity = action.view_affinity();
```
