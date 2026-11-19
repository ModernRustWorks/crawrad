---
id: adapter-model-modeladapter-new
kind: doc
target_file: src/adapter/model.rs
line_hint: 15
author: 
created: 
title: "ModelAdapter::new"
tags: []
signature: "pub fn new (initial : M) -> Self"
---

Create a new model adapter wrapping the given initial model instance.

# Examples

```rust
use crawrad_binding::ModelAdapter;

let adapter = ModelAdapter::new("hello".to_string());
assert_eq!(adapter.get(), "hello");
```
