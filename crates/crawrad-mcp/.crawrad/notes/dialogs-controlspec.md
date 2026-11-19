---
id: dialogs-controlspec
kind: doc
target_file: src/dialogs.rs
line_hint: 32
author: 
created: 
title: "ControlSpec"
tags: []
signature: "pub struct ControlSpec"
---

Specification for declaring dialog controls.

One control, or `count` controls named `name` with `{i}` = 0, 1, …; a row
names in `text_of` the label inside it whose text is reported.

# Examples

```rust
use crawrad_mcp::dialogs::{ControlKind, ControlSpec};

let spec = ControlSpec {
    name: "btn",
    kind: ControlKind::Button,
    count: 1,
    text_of: "",
};
assert_eq!(spec.count, 1);
```
