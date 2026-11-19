---
id: targets-egui-state-inits-of
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 1022
author: 
created: 
title: "inits_of"
tags: []
signature: "pub fn inits_of < 'a > (node : & Node , desc : & ComponentDesc , doc : impl Into < Option < & 'a Document > > ,) -> Vec < (String , String) >"
---

Computes the list of `(field_name, literal)` initialisers for a component node.

# Examples

```rust
use crawrad_codegen::targets::egui::state::inits_of;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let node = Node::new("btn", "Button");
let inits = inits_of(&node, desc, None);
assert!(!inits.is_empty());
```
