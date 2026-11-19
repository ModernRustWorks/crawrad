---
id: components-page-control-tab-strip-top
kind: doc
target_file: src/components/page_control.rs
line_hint: 69
author: 
created: 
title: "tab_strip_top"
tags: []
signature: "pub fn tab_strip_top (h : f32 , bottom : bool) -> f32"
---

The top coordinate of the tab strip in a page control.

The top of the tab strip in a page control `h` points tall: 0, or its
bottom edge minus the strip when the tabs sit at the bottom.

# Examples

```rust
use crawrad_registry::tab_strip_top;

assert_eq!(tab_strip_top(100.0, false), 0.0);
```
