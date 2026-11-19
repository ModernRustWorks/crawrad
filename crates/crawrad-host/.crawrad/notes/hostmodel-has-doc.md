---
id: hostmodel-has-doc
kind: doc
target_file: src/lib.rs
line_hint: 187
author: 
created: 
title: "HostModel::has_doc"
tags: []
signature: "pub fn has_doc (& self) -> bool"
---

Returns `true` if a document is currently open.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(!host.has_doc());
```
