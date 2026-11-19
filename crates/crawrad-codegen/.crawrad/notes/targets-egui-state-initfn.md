---
id: targets-egui-state-initfn
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 528
author: 
created: 
title: "InitFn"
tags: []
signature: "pub type InitFn = fn (& Node , Option < & Document >) -> String"
---

Function signature for computed property initialisers.

# Examples

```rust
use crawrad_codegen::targets::egui::state::InitFn;
use crawrad_ir::Node;

let f: InitFn = |n, _| n.name().to_string();
let n = Node::new("btn", "Button");
assert_eq!(f(&n, None), "btn");
```
