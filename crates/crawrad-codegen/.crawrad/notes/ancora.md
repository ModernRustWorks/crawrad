---
id: ancora
kind: doc
target_file: src/lib.rs
line_hint: 127
author: 
created: 
title: "Ancora"
tags: []
signature: "pub struct Ancora"
---

Where to insert a new handler inside the user's file.

# Examples

```rust
use crawrad_codegen::Ancora;

let ancora = Ancora {
    marca: "// [handlers]".to_string(),
    texto: "fn on_click(&mut self) {}".to_string(),
};
assert_eq!(ancora.marca, "// [handlers]");
```
