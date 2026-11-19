---
id: controller-shownvariable
kind: doc
target_file: src/controller.rs
line_hint: 62
author: 
created: 
title: "ShownVariable"
tags: []
signature: "pub struct ShownVariable"
---

A variable formatted for display in the IDE.

A variable as the IDE shows it: the presenter's text, and the reference to
expand it (`0` when it has no children).

# Examples

```rust
use crawrad_dap::ShownVariable;

let var = ShownVariable {
    name: "x".into(),
    value: "42".into(),
    type_name: Some("i32".into()),
    reference: 0,
};
assert_eq!(var.name, "x");
```
