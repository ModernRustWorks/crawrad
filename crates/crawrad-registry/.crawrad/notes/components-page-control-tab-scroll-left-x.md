---
id: components-page-control-tab-scroll-left-x
kind: doc
target_file: src/components/page_control.rs
line_hint: 59
author: 
created: 
title: "tab_scroll_left_x"
tags: []
signature: "pub fn tab_scroll_left_x (width : f32 , plus : bool) -> f32"
---

Left edge of the scroll buttons relative to the control's left.

Left edge (relative to the control's left) of the `<` button; the `>` button
is right after it. Only meaningful when `TabWindow::arrows`.

# Examples

```rust
use crawrad_registry::tab_scroll_left_x;

let x = tab_scroll_left_x(200.0, false);
assert!(x > 0.0);
```
