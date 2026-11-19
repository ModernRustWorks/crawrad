---
id: hostmodel-doc
kind: doc
target_file: src/lib.rs
line_hint: 153
author: 
created: 
title: "HostModel::doc"
tags: []
signature: "pub fn doc (& self) -> Option < & Doc >"
---

Returns a reference to the active document, if any.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(host.doc().is_none());
```
