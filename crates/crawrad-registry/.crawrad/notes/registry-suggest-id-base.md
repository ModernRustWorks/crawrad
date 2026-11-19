---
id: registry-suggest-id-base
kind: doc
target_file: src/lib.rs
line_hint: 734
author: 
created: 
title: "Registry::suggest_id_base"
tags: []
signature: "pub fn suggest_id_base (& self , base : & str , existing : impl Fn (& str) -> bool) -> String"
---

Same, but from an arbitrary base name instead of class.

# Examples

```rust
use crawrad_registry::Registry;

let reg = Registry::builtin();
let id = reg.suggest_id_base("panel", |name| name == "panel1");
assert_eq!(id, "panel2");
```
