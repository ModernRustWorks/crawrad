---
id: capability-capabilityparam
kind: doc
target_file: src/capability.rs
line_hint: 37
author: 
created: 
title: "CapabilityParam"
tags: []
signature: "pub struct CapabilityParam"
---

Typed parameter of a capability, with metadata and JSON Schema support.

# Examples

```rust
use crawrad_actions::capability::CapabilityParam;
use crawrad_actions::ParamKind;

let param = CapabilityParam::new("name", ParamKind::String, "A name");
assert_eq!(param.name, "name");
```
