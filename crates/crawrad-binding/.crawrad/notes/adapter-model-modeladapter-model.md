---
id: adapter-model-modeladapter-model
kind: doc
target_file: src/adapter/model.rs
line_hint: 22
author: 
created: 
title: "ModelAdapter::model"
tags: []
signature: "pub fn model (& self) -> & Observable < M >"
---

Access the underlying model observable.

# Examples

```rust
use crawrad_binding::ModelAdapter;

let adapter = ModelAdapter::new(10);
assert_eq!(adapter.model().get(), 10);
```
