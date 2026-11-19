---
id: protocol-variable
kind: doc
target_file: src/protocol.rs
line_hint: 53
author: 
created: 
title: "Variable"
tags: []
signature: "pub struct Variable"
---

One entry of a `variables` response, the unit a presenter formats.

# Examples

```rust
use crawrad_dap::Variable;

let var = Variable {
    name: "x".into(),
    value: "10".into(),
    type_name: Some("i32".into()),
    variables_reference: 0,
    evaluate_name: None,
    memory_reference: None,
};
assert_eq!(var.name, "x");
```
