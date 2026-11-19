---
id: transformacao
kind: doc
target_file: src/lib.rs
line_hint: 154
author: 
created: 
title: "Transformacao"
tags: []
signature: "pub struct Transformacao"
---

Canvas coordinate transformation between design units and screen points.

# Examples

```rust
use crawrad_runtime_makepad::Transformacao;

let t = Transformacao::new(10.0, 20.0, 2.0);
assert_eq!(t.screen_point(5.0, 5.0), (20.0, 30.0));
```
