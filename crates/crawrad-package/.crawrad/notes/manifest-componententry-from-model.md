---
id: manifest-componententry-from-model
kind: doc
target_file: src/manifest.rs
line_hint: 935
author: 
created: 
title: "ComponentEntry::from_model"
tags: []
signature: "pub fn from_model (c : & ExternalComponent) -> Self"
---

The model component, in the file format.

# Examples

```rust
use crawrad_registry::Registry;
use crawrad_registry::external::ExternalComponent;
use crawrad_package::manifest::ComponentEntry;

let reg = Registry::builtin();
let d = reg.all().next().unwrap();
let ext = ExternalComponent::from(d);
let entry = ComponentEntry::from_model(&ext);
assert_eq!(entry.class, ext.class);
```
