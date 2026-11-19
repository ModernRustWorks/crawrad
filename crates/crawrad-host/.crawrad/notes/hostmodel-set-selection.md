---
id: hostmodel-set-selection
kind: doc
target_file: src/lib.rs
line_hint: 197
author: 
created: 
title: "HostModel::set_selection"
tags: []
signature: "pub fn set_selection (& mut self , selection : Vec < NodeId >)"
---

Replaces the active selection in the document.

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::new();
host.set_selection(vec![]);
assert!(host.selection().is_empty());
```
