---
id: targets-makepad-components-text-path
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 136
author: 
created: 
title: "text_path"
tags: []
signature: "pub fn text_path (node : & Node , field : & str) -> String"
---

Where `node`'s keyboard-and-text widget lives under its `field` (#139).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::text_path;
use crawrad_ir::Node;

let node = Node::new("ed", "Edit");
assert_eq!(text_path(&node, "ed"), "ed");
```
