---
id: props-props-vert-scroll-bar
kind: doc
target_file: src/props.rs
line_hint: 976
author: 
created: 
title: "Props::vert_scroll_bar"
tags: []
signature: "pub fn vert_scroll_bar (default : bool) -> PropDesc"
---

Vertical scroll bar enabled (`VertScrollBar`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::vert_scroll_bar(true);
assert_eq!(p.key, "vert_scroll_bar");
```
