---
id: capability-capabilityparam-new
kind: doc
target_file: src/capability.rs
line_hint: 67
author: 
created: 
title: "CapabilityParam::new"
tags: []
signature: "pub fn new (name : impl Into < String > , kind : ParamKind , description : impl Into < String >) -> Self"
---

Creates a new custom parameter.

# Examples

```rust
use crawrad_actions::capability::CapabilityParam;
use crawrad_actions::ParamKind;

let param = CapabilityParam::new("limit", ParamKind::Integer, "Limit of results");
assert!(param.required);
```
