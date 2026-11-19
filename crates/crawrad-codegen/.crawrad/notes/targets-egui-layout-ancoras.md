---
id: targets-egui-layout-ancoras
kind: doc
target_file: src/targets/egui/layout.rs
line_hint: 45
author: 
created: 
title: "ancoras"
tags: []
signature: "pub fn ancoras (n : & Node) -> String"
---

The node's anchors, as a literal of the emitted type.

# Examples

```rust
use crawrad_codegen::targets::egui::layout::ancoras;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
let anc = ancoras(&node);
assert!(anc.contains("Anchors"));
```
