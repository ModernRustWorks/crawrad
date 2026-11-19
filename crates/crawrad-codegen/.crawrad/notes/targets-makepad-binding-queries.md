---
id: targets-makepad-binding-queries
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 270
author: 
created: 
title: "queries"
tags: []
signature: "pub fn queries () -> Vec < & 'static str >"
---

As consultas distintas que a tabela usa.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::queries;

let qs = queries();
assert!(qs.contains(&"clicked"));
```
