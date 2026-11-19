---
id: props-props-scroll-bar-kind
kind: doc
target_file: src/props.rs
line_hint: 872
author: 
created: 
title: "Props::scroll_bar_kind"
tags: []
signature: "pub fn scroll_bar_kind () -> PropDesc"
---

Scroll bar orientation (`Kind`: `sbHorizontal` or `sbVertical`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::scroll_bar_kind();
assert_eq!(p.key, "kind");
```
