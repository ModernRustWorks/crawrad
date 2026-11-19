---
id: hostmodel-center
kind: doc
target_file: src/lib.rs
line_hint: 317
author: 
created: 
title: "HostModel::center"
tags: []
signature: "pub fn center (& self) -> ViewCenter"
---

Returns which panel is currently in front.

# Examples

```rust
use crawrad_host::{HostModel, ViewCenter};

let host = HostModel::new();
assert_eq!(host.center(), ViewCenter::Design);
```
