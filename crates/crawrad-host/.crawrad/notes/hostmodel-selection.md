---
id: hostmodel-selection
kind: doc
target_file: src/lib.rs
line_hint: 192
author: 
created: 
title: "HostModel::selection"
tags: []
signature: "pub fn selection (& self) -> & [NodeId]"
---

Returns the active selection node IDs from the document.

Returns the active selection node IDs from the document, or empty slice if no document.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(host.selection().is_empty());
```
