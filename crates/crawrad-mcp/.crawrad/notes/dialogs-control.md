---
id: dialogs-control
kind: doc
target_file: src/dialogs.rs
line_hint: 68
author: 
created: 
title: "Control"
tags: []
signature: "pub struct Control"
---

An expanded control descriptor within a dialog.

One control as the dialog has it: its widget name, its kind, and for a row
the label inside it that carries its text.

# Examples

```rust
use crawrad_mcp::dialogs::{Control, ControlKind};

let c = Control {
    name: "btn".into(),
    kind: ControlKind::Button,
    text_of: None,
};
assert_eq!(c.name, "btn");
```
