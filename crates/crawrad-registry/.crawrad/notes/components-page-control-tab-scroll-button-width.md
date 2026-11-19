---
id: components-page-control-tab-scroll-button-width
kind: doc
target_file: src/components/page_control.rs
line_hint: 14
author: 
created: 
title: "TAB_SCROLL_BUTTON_WIDTH"
tags: []
signature: "pub const TAB_SCROLL_BUTTON_WIDTH : f32 = 16.0"
---

Width in points of each tab strip scroll button.

Width of each of the two scroll buttons (`<` `>`) the strip shows when its
tabs do not fit (#13), in points.

# Examples

```rust
use crawrad_registry::TAB_SCROLL_BUTTON_WIDTH;

assert_eq!(TAB_SCROLL_BUTTON_WIDTH, 16.0);
```
