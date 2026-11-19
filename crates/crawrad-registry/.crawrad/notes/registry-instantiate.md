---
id: registry-instantiate
kind: doc
target_file: src/lib.rs
line_hint: 707
author: 
created: 
title: "Registry::instantiate"
tags: []
signature: "pub fn instantiate (& self , class : & str , id : impl Into < String > , at : Rect) -> Option < Node >"
---

Creates a new node with descriptor default values.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let node = reg.instantiate("Button", "btn1", Rect::new(0.0, 0.0, 80.0, 24.0));
assert!(node.is_some());
```
