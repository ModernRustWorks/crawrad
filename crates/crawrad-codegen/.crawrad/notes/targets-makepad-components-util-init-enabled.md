---
id: targets-makepad-components-util-init-enabled
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 38
author: 
created: 
title: "init_enabled"
tags: []
signature: "pub fn init_enabled (node : & Node) -> String"
---

Emits the initialization string for the `enabled` property.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::init_enabled;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
assert_eq!(init_enabled(&node), "enabled: true");
```
