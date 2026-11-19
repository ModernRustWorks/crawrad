---
id: capability-capabilityparam-from-param
kind: doc
target_file: src/capability.rs
line_hint: 55
author: 
created: 
title: "CapabilityParam::from_param"
tags: []
signature: "pub fn from_param (p : & Param) -> Self"
---

Builds from a native static [`Param`].

# Examples

```rust
use crawrad_actions::capability::CapabilityParam;
use crawrad_actions::{Param, ParamKind};

let p = Param {
    name: "count",
    kind: ParamKind::Integer,
    description: "Item count",
    list: false,
    required: true,
    domain: None,
};
let cp = CapabilityParam::from_param(&p);
assert_eq!(cp.name, "count");
```
