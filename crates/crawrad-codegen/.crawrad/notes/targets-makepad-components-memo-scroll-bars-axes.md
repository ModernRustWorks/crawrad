---
id: targets-makepad-components-memo-scroll-bars-axes
kind: doc
target_file: src/targets/makepad/components/memo.rs
line_hint: 195
author: 
created: 
title: "scroll_bars_axes"
tags: []
signature: "pub fn scroll_bars_axes (node : & Node) -> (bool , bool)"
---

The horizontal and vertical scroll bar visibility flags for `node`.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::memo::scroll_bars_axes;
use crawrad_ir::Node;

let node = Node::new("memo", "Memo");
assert_eq!(scroll_bars_axes(&node), (false, false));
```
