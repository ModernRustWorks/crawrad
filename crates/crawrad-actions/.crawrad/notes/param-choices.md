---
id: param-choices
kind: doc
target_file: src/lib.rs
line_hint: 439
author: 
created: 
title: "Param::choices"
tags: []
signature: "pub fn choices (& self) -> Vec < Choice >"
---

Every value this parameter accepts, or empty when it takes free text.

# Examples

```rust
use crawrad_actions::{Param, ParamKind};

let p = Param {
    name: "name",
    kind: ParamKind::String,
    description: "desc",
    list: false,
    required: false,
    domain: None,
};
assert!(p.choices().is_empty());
```
