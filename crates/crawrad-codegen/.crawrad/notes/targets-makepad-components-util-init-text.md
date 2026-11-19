---
id: targets-makepad-components-util-init-text
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 44
author: 
created: 
title: "init_text"
tags: []
signature: "pub fn init_text (node : & Node) -> String"
---

Emits the initialization string for the `text` property.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::init_text;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
assert!(init_text(&node).starts_with("text: "));
```
