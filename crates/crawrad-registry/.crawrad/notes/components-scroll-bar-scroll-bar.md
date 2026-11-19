---
id: components-scroll-bar-scroll-bar
kind: doc
target_file: src/components/scroll_bar.rs
line_hint: 4
author: 
created: 
title: "scroll_bar"
tags: []
signature: "pub fn scroll_bar () -> ComponentDesc"
---

`ScrollBar`: standalone scroll bar from the classic RAD IDE (`TScrollBar`).

# Examples

```rust
use crawrad_registry::scroll_bar;

let desc = scroll_bar();
assert_eq!(desc.class, "ScrollBar");
```
