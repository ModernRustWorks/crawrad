---
id: components-page-control-plus-rectangle
kind: doc
target_file: src/components/page_control.rs
line_hint: 37
author: 
created: 
title: "plus_rectangle"
tags: []
signature: "pub fn plus_rectangle (r : RectIr) -> RectIr"
---

The rectangle of the button to add a page on the tab strip.

The rectangle of the "+" that creates a page — against the RIGHT edge, and not
after the last tab (`crawrad-designer/src/lib.rs:459-462`).

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime_makepad::components::page_control::{plus_rectangle, LARGURA_DO_MAIS};

let r = Rect::new(0.0, 0.0, 200.0, 100.0);
let plus = plus_rectangle(r);
assert_eq!(plus.w, LARGURA_DO_MAIS);
```
