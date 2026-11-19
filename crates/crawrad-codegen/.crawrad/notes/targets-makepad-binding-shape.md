---
id: targets-makepad-binding-shape
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 38
author: 
created: 
title: "Shape"
tags: []
signature: "pub enum Shape"
---

The shape of the query accepted by the framework.

Three, and each one came from an `emit_actions` that exists today. There is no
fourth because no class needed it.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::Shape;

let s = Shape::Plain;
assert_eq!(s, Shape::Plain);
```
