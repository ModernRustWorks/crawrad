---
id: emission-emission
kind: doc
target_file: src/emission.rs
line_hint: 45
author: 
created: 
title: "Emission"
tags: []
signature: "pub enum Emission"
---

Uma coisa a dizer sobre um objeto.

# Examples

```rust
use crawrad_codegen::emission::Emission;

let em = Emission::Create {
    id: "btn".to_string(),
    class: "Button".to_string(),
};
assert!(matches!(em, Emission::Create { .. }));
```
