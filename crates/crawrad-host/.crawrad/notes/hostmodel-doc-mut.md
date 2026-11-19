---
id: hostmodel-doc-mut
kind: doc
target_file: src/lib.rs
line_hint: 158
author: 
created: 
title: "HostModel::doc_mut"
tags: []
signature: "pub fn doc_mut (& mut self) -> Option < & mut Doc >"
---

Returns a mutable reference to the active document, if any.

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::new();
assert!(host.doc_mut().is_none());
```
