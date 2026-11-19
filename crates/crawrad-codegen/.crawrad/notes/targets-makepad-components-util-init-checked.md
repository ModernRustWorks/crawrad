---
id: targets-makepad-components-util-init-checked
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 51
author: 
created: 
title: "init_checked"
tags: []
signature: "pub fn init_checked (node : & Node) -> String"
---

Emits the initialization string for the `checked` property.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::init_checked;
use crawrad_ir::Node;

let node = Node::new("chk", "CheckBox");
assert_eq!(init_checked(&node), "checked: false");
```
