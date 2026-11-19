---
id: generator-set-active-target
kind: doc
target_file: src/generator.rs
line_hint: 112
author: 
created: 
title: "set_active_target"
tags: []
signature: "pub fn set_active_target (target : crawrad_codegen :: AlvoGerador)"
---

Change the generator target in force.

# Examples

```rust
use crawrad_codegen::AlvoGerador;
use crawrad_project::generator::{active_target, set_active_target};

set_active_target(AlvoGerador::Makepad);
assert_eq!(active_target(), AlvoGerador::Makepad);
```
