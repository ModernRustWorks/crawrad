---
id: external-externalcomponent-into-catalogue
kind: doc
target_file: src/external.rs
line_hint: 395
author: 
created: 
title: "ExternalComponent::into_catalogue"
tags: []
signature: "pub fn into_catalogue (self) -> ComponentDesc"
---

Converts the owned external component into a catalog descriptor.

**The descriptor the catalogue can hold.**

It consumes `self` because everything inside it becomes `'static` — see
the module header. Keeping the owned copy alive afterwards would be two
truths about the same component, free to drift.

# Examples

```rust
use crawrad_registry::button;
use crawrad_registry::external::ExternalComponent;

let ext = ExternalComponent::from(&button());
let desc = ext.into_catalogue();
assert_eq!(desc.class, "Button");
```
