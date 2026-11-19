---
id: hostmodel-set-center
kind: doc
target_file: src/lib.rs
line_hint: 322
author: 
created: 
title: "HostModel::set_center"
tags: []
signature: "pub fn set_center (& mut self , center : ViewCenter)"
---

Sets which panel is in front.

# Examples

```rust
use crawrad_host::{HostModel, ViewCenter};

let mut host = HostModel::new();
host.set_center(ViewCenter::Code);
assert_eq!(host.center(), ViewCenter::Code);
```
