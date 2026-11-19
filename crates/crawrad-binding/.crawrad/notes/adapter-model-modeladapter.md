---
id: adapter-model-modeladapter
kind: doc
target_file: src/adapter/model.rs
line_hint: 8
author: 
created: 
title: "ModelAdapter"
tags: []
signature: "pub struct ModelAdapter < M >"
---

Reactive adapter wrapping a domain struct into observable bindings.

Reactive adapter wrapping any domain struct `M` and projecting its fields into
two-way observable bindings.

# Examples

```rust
use crawrad_binding::ModelAdapter;

let adapter = ModelAdapter::new(42);
assert_eq!(adapter.get(), 42);
```
