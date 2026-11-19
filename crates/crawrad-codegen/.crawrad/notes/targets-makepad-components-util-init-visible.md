---
id: targets-makepad-components-util-init-visible
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 32
author: 
created: 
title: "init_visible"
tags: []
signature: "pub fn init_visible (node : & Node) -> String"
---

Emits the initialization string for the `visible` property.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::init_visible;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
assert_eq!(init_visible(&node), "visible: true");
```
