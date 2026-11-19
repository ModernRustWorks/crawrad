---
id: targets-egui-state-init-of
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 1062
author: 
created: 
title: "init_of"
tags: []
signature: "pub fn init_of < 'a > (node : & Node , desc : & ComponentDesc , doc : impl Into < Option < & 'a Document > > ,) -> String"
---

Emits the `Class { field: literal, ... }` initialiser for a component node.

# Examples

```rust
use crawrad_codegen::targets::egui::state::init_of;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let node = Node::new("btn", "Button");
let init = init_of(&node, desc, None);
assert!(init.contains("Button {"));
```
