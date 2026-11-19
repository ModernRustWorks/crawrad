---
id: external-externalcomponent
kind: doc
target_file: src/external.rs
line_hint: 295
author: 
created: 
title: "ExternalComponent"
tags: []
signature: "pub struct ExternalComponent"
---

A whole component declared from outside.

**A whole component, declared from outside.**

Field for field the same questions [`ComponentDesc`] answers. A field that
exists there and not here is a field a third-party component could never
have — which is why the round-trip gate compares the two whole.

# Examples

```rust
use crawrad_registry::button;
use crawrad_registry::external::ExternalComponent;

let ext = ExternalComponent::from(&button());
assert_eq!(ext.class, "Button");
```
