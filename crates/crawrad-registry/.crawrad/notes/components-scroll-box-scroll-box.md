---
id: components-scroll-box-scroll-box
kind: doc
target_file: src/components/scroll_box.rs
line_hint: 4
author: 
created: 
title: "scroll_box"
tags: []
signature: "pub fn scroll_box () -> ComponentDesc"
---

`ScrollBox`: scrolling container from the classic RAD IDE (`TScrollBox`).

# Examples

```rust
use crawrad_registry::scroll_box;

let desc = scroll_box();
assert_eq!(desc.class, "ScrollBox");
```
