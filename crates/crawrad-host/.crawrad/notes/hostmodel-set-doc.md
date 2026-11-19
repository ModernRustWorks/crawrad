---
id: hostmodel-set-doc
kind: doc
target_file: src/lib.rs
line_hint: 163
author: 
created: 
title: "HostModel::set_doc"
tags: []
signature: "pub fn set_doc (& mut self , doc : Option < Doc >)"
---

Sets or clears the active document.

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::new();
host.set_doc(None);
assert!(!host.has_doc());
```
