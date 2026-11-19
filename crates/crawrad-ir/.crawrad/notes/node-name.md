---
id: node-name
kind: doc
target_file: src/lib.rs
line_hint: 132
author: 
created: 
title: "Node::name"
tags: []
signature: "pub fn name (& self) -> & str"
---

The `Name` of the component.

The `Name` of the component — the label that the user sees and edits, and that
codegen turns into a field. Falls back to `id` when the property does not exist, which
is the case for a newly constructed node in tests.

# Examples

```rust
use crawrad_ir::Node;

let mut node = Node::new("btn1", "Button");
assert_eq!(node.name(), "btn1");
node.set_name("btn_ok");
assert_eq!(node.name(), "btn_ok");
```
