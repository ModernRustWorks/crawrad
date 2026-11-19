---
id: targets-egui-control-keys-dispatch
kind: doc
target_file: src/targets/egui/control_keys.rs
line_hint: 29
author: 
created: 
title: "dispatch"
tags: []
signature: "pub fn dispatch (n : & Node) -> Result < String , Erro >"
---

Dispatches control key events to configured handlers.

The block that hands the control's key events to its handlers, for the
control's `res` (its `egui::Response`); empty when none is wired.

`lost_focus` counts as focus: a single-line edit lets go of the focus on
the very frame Enter is pressed, and that Enter is still its key.

# Examples

```rust
use crawrad_codegen::targets::egui::control_keys::dispatch;
use crawrad_ir::Node;

let node = Node::new("ed", "Edit");
let code = dispatch(&node).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if an identifier name is invalid.
