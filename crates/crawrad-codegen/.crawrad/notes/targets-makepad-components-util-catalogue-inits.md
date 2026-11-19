---
id: targets-makepad-components-util-catalogue-inits
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 185
author: 
created: 
title: "catalogue_inits"
tags: []
signature: "pub fn catalogue_inits (node : & Node , desc : & ComponentDesc , by_hand : & [& str]) -> Vec < String >"
---

Returns initializers for catalogue fields of a component node.

The initializers of those same fields for `node`: its value when it has one, the descriptor's default otherwise.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::catalogue_inits;
use crawrad_ir::Node;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let node = Node::new("btn", "Button");
let inits = catalogue_inits(&node, desc, &["text"]);
assert!(!inits.is_empty());
```
