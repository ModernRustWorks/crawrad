---
id: key-routing-comando
kind: doc
target_file: src/key_routing.rs
line_hint: 73
author: 
created: 
title: "Comando"
tags: []
signature: "pub enum Comando"
---

What a key became.

Two families, and the separation is the same as the egui shell (`main.rs:7791`):
what has an id in the catalog, and the gesture that does not.

# Examples

```rust
use crawrad_actions::key_routing::Comando;

let cmd = Comando::Acao("file.save");
assert_eq!(cmd, Comando::Acao("file.save"));
```
