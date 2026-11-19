---
id: targets-egui-control-keys-wants
kind: doc
target_file: src/targets/egui/control_keys.rs
line_hint: 23
author: 
created: 
title: "wants"
tags: []
signature: "pub fn wants (n : & Node) -> bool"
---

Checks whether a control dispatches its own key events.

Whether this control dispatches a key event: its class is listed and a
handler is wired for `OnKeyDown` or `OnKeyUp`.

# Examples

```rust
use crawrad_codegen::targets::egui::control_keys::wants;
use crawrad_ir::Node;

let node = Node::new("ed", "Edit");
assert!(!wants(&node));
```
