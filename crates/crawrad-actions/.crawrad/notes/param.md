---
id: param
kind: doc
target_file: src/lib.rs
line_hint: 344
author: 
created: 
title: "Param"
tags: []
signature: "pub struct Param"
---

The description of a parameter — what the MCP publishes as a signature.

# Examples

```rust
use crawrad_actions::{Param, ParamKind};

let p = Param {
    name: "name",
    kind: ParamKind::String,
    description: "Component name",
    list: false,
    required: true,
    domain: None,
};
assert_eq!(p.name, "name");
```
