---
id: external-externalrequirement
kind: doc
target_file: src/external.rs
line_hint: 336
author: 
created: 
title: "ExternalRequirement"
tags: []
signature: "pub struct ExternalRequirement"
---

A dependency that the generated project gains because of this component.

The entire reason is in the doc of [`crate::CrateRequirement`]: it is the
classic RAD *driver link*, and in Rust it is a Cargo feature.

# Examples

```rust
use crawrad_registry::external::ExternalRequirement;

let req = ExternalRequirement {
    crate_name: "rusqlite".into(),
    version: "0.29".into(),
    features: vec!["bundled".into()],
    default_features: true,
};
assert_eq!(req.crate_name, "rusqlite");
```
