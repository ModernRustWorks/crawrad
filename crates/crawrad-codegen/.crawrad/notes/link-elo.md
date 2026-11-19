---
id: link-elo
kind: doc
target_file: src/link.rs
line_hint: 436
author: 
created: 
title: "Elo"
tags: []
signature: "pub struct Elo"
---

O evento que o duplo clique abre, e o nome da `fn` dele.

# Examples

```rust
use crawrad_codegen::link::Elo;

let elo = Elo {
    evento: "on_click".to_string(),
    handler: "btn_click".to_string(),
    ja_ligado: false,
};
assert_eq!(elo.evento, "on_click");
```
