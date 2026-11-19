---
id: designer-with-registry
kind: doc
target_file: src/lib.rs
line_hint: 238
author: 
created: 
title: "Designer::with_registry"
tags: []
signature: "pub fn with_registry (registry : std :: rc :: Rc < Registry >) -> Self"
---

Creates a designer with a component catalog.

Designer with component catalog — required for typographic
alignment, which depends on the metrics of each class.

# Examples

```rust
use std::rc::Rc;
use crawrad_designer::Designer;
use crawrad_registry::Registry;

let d = Designer::with_registry(Rc::new(Registry::builtin()));
assert!(d.registry.is_some());
```
