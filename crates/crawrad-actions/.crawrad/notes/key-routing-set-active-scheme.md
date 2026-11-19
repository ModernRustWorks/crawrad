---
id: key-routing-set-active-scheme
kind: doc
target_file: src/key_routing.rs
line_hint: 64
author: 
created: 
title: "set_active_scheme"
tags: []
signature: "pub fn set_active_scheme (esquema : KeymapScheme)"
---

Define o esquema de atalhos de teclado ativo na casca.

# Examples

```rust
use crawrad_actions::key_routing::{active_scheme, set_active_scheme};
use crawrad_actions::KeymapScheme;

set_active_scheme(KeymapScheme::Classic);
assert_eq!(active_scheme(), KeymapScheme::Classic);
set_active_scheme(KeymapScheme::VsCode);
```
