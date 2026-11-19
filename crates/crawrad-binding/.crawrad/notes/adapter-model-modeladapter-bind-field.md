---
id: adapter-model-modeladapter-bind-field
kind: doc
target_file: src/adapter/model.rs
line_hint: 41
author: 
created: 
title: "ModelAdapter::bind_field"
tags: []
signature: "pub fn bind_field < T , G , S > (& self , getter : G , setter : S) -> Observable < T > where T : Clone + PartialEq + Send + Sync + 'static , G : Fn (& M) -> T + Send + Sync + 'static , S : Fn (& mut M , T) + Send + Sync + 'static ,"
---

Bind a single property of `M` to an [`Observable<T>`].

Changes in the UI will update the field in `M` through `setter`, and
external changes to `M` will update the field observable through `getter`.

# Examples

```rust
use crawrad_binding::ModelAdapter;

#[derive(Clone, PartialEq)]
struct Person { name: String }

let adapter = ModelAdapter::new(Person { name: "Alice".to_string() });
let name = adapter.bind_field(|p| p.name.clone(), |p, v| p.name = v);
assert_eq!(name.get(), "Alice");
name.set("Bob".to_string());
assert_eq!(adapter.get().name, "Bob");
```
