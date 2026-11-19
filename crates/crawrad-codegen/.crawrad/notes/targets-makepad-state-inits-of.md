---
id: targets-makepad-state-inits-of
kind: doc
target_file: src/targets/makepad/state.rs
line_hint: 239
author: 
created: 
title: "inits_of"
tags: []
signature: "pub fn inits_of (node : & Node , desc : & ComponentDesc) -> Vec < (String , String) >"
---

The initializers of a node's state, as `(campo, literal)`.

**The initializers of a node's state**, as `(campo, literal)`.

# Examples

```rust
use crawrad_codegen::targets::makepad::state::inits_of;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let node = Node::new("btn", "Button");
let inits = inits_of(&node, desc);
assert!(!inits.is_empty());
```
