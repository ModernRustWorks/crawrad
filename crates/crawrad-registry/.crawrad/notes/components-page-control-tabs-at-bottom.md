---
id: components-page-control-tabs-at-bottom
kind: doc
target_file: src/components/page_control.rs
line_hint: 64
author: 
created: 
title: "tabs_at_bottom"
tags: []
signature: "pub fn tabs_at_bottom (node : & crawrad_ir :: Node) -> bool"
---

Returns whether the tabs of a node sit at the bottom.

Whether the tabs of `node` sit at the bottom (`TabPosition = tpBottom`,
#449). One answer for the designer, the canvas and both targets.

# Examples

```rust
use crawrad_ir::Node;
use crawrad_registry::tabs_at_bottom;

let node = Node::new("pgc1", "PageControl");
assert!(!tabs_at_bottom(&node));
```
