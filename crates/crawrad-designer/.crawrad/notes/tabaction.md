---
id: tabaction
kind: doc
target_file: src/lib.rs
line_hint: 228
author: 
created: 
title: "TabAction"
tags: []
signature: "pub enum TabAction"
---

What to do after a click on the tab strip of a `PageControl`.

# Examples

```rust
use crawrad_designer::TabAction;
use crawrad_ir::NodeId;

let action = TabAction::Create { control: NodeId::from("pc") };
assert!(matches!(action, TabAction::Create { .. }));
```
