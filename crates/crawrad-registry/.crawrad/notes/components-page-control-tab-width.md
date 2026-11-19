---
id: components-page-control-tab-width
kind: doc
target_file: src/components/page_control.rs
line_hint: 8
author: 
created: 
title: "TAB_WIDTH"
tags: []
signature: "pub const TAB_WIDTH : f32 = 92.0"
---

Width of a tab, in points. **Fixed**, and this is a choice.

The classic RAD IDE sizes the tab by text. Here we cannot without paying dearly: the
designer resolves clicks on the MODEL and has no font metrics — it does not
depend on egui, and that independence is what keeps the model testable without
screen. Variable width tabs would require measuring text in the designer and the
painter, and having the two always agree.

Fixed width trades fidelity for a better guarantee: where drawing puts the
tab is where the click finds it, by construction. The way out, when worth
it, is a shared metric service — not two measurements.

# Examples

```rust
use crawrad_registry::TAB_WIDTH;

assert_eq!(TAB_WIDTH, 92.0);
```
