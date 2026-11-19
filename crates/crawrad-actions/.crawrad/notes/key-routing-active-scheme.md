---
id: key-routing-active-scheme
kind: doc
target_file: src/key_routing.rs
line_hint: 56
author: 
created: 
title: "active_scheme"
tags: []
signature: "pub fn active_scheme () -> KeymapScheme"
---

Retorna o esquema de atalhos de teclado ativo na casca.

# Examples

```rust
use crawrad_actions::key_routing::active_scheme;
use crawrad_actions::KeymapScheme;

let scheme = active_scheme();
assert!(scheme == KeymapScheme::VsCode || scheme == KeymapScheme::Classic);
```
