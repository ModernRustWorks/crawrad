---
id: targets-makepad-layout-str-val
kind: doc
target_file: src/targets/makepad/layout.rs
line_hint: 21
author: 
created: 
title: "str_val"
tags: []
signature: "pub fn str_val < 'a > (n : & 'a Node , key : & str) -> Option < & 'a str >"
---

Retrieves a string property from a node.

# Examples

```rust
use crawrad_codegen::targets::makepad::layout::str_val;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
assert_eq!(str_val(&node, "caption"), None);
```
