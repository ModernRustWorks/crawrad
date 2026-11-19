---
id: hostmodel-select-one
kind: doc
target_file: src/lib.rs
line_hint: 204
author: 
created: 
title: "HostModel::select_one"
tags: []
signature: "pub fn select_one (& mut self , id : impl Into < NodeId >)"
---

Selects a single node in the document.

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::new();
host.select_one("btn");
```
