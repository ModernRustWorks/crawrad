---
id: designer-tool-wired
kind: doc
target_file: src/designer_tool.rs
line_hint: 276
author: 
created: 
title: "Wired"
tags: []
signature: "pub struct Wired"
---

What `designer.wire_event` bound: the shell then writes the stub.

# Examples

```rust
use crawrad_mcp::designer_tool::Wired;

let w = Wired {
    node: "btn1".into(),
    name: "btn1".into(),
    event: "OnClick".into(),
    handler: "btn1_click".into(),
};
assert_eq!(w.node, "btn1");
```
