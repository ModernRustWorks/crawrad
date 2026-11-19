---
id: adapter-model-modeladapter-get
kind: doc
target_file: src/adapter/model.rs
line_hint: 29
author: 
created: 
title: "ModelAdapter::get"
tags: []
signature: "pub fn get (& self) -> M"
---

Retrieve a snapshot of the current model value.

# Examples

```rust
use crawrad_binding::ModelAdapter;

let adapter = ModelAdapter::new(100);
assert_eq!(adapter.get(), 100);
```
