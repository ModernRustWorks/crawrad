---
id: hostmodel-clear-selection
kind: doc
target_file: src/lib.rs
line_hint: 306
author: 
created: 
title: "HostModel::clear_selection"
tags: []
signature: "pub fn clear_selection (& mut self)"
---

Clears the active selection in the document.

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::new();
host.clear_selection();
assert!(host.selection().is_empty());
```
