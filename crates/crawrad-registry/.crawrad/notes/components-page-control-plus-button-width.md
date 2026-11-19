---
id: components-page-control-plus-button-width
kind: doc
target_file: src/components/page_control.rs
line_hint: 11
author: 
created: 
title: "PLUS_BUTTON_WIDTH"
tags: []
signature: "pub const PLUS_BUTTON_WIDTH : f32 = 22.0"
---

Width of the "+" button on the tab strip, in points.

It sits **pinned to the right edge** of the control, not after the
last tab. The previous version placed it at `n * TAB_WIDTH`, and with 92pt
tabs on a 320 control the fifth tab started at 368 — outside the
control. As "+" is the only way to create a page, `PageControl`
was stuck at four. With three tabs it was already drawn half
outside the edge.

On the right it is always reachable, with any number of tabs.

# Examples

```rust
use crawrad_registry::PLUS_BUTTON_WIDTH;

assert_eq!(PLUS_BUTTON_WIDTH, 22.0);
```
