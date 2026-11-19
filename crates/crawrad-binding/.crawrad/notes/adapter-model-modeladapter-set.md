---
id: adapter-model-modeladapter-set
kind: doc
target_file: src/adapter/model.rs
line_hint: 36
author: 
created: 
title: "ModelAdapter::set"
tags: []
signature: "pub fn set (& self , new_model : M) -> bool"
---

Set a new whole-model instance, notifying all registered field bindings.

# Examples

```rust
use crawrad_binding::ModelAdapter;

let adapter = ModelAdapter::new(10);
assert!(adapter.set(20));
assert_eq!(adapter.get(), 20);
```
