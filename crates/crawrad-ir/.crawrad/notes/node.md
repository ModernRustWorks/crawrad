---
id: node
kind: doc
target_file: src/lib.rs
line_hint: 72
author: 
created: 
title: "Node"
tags: []
signature: "pub struct Node"
---

A component in the tree.

`class` is a string, not an enum: third-party components enter through the
registry without touching the IR. The old core closed this in an enum of three
controls — see §3.8 of the study.

# Examples

```rust
use crawrad_ir::Node;

let node = Node::new("btn_ok", "Button");
assert_eq!(node.id, "btn_ok");
assert_eq!(node.class, "Button");
```
