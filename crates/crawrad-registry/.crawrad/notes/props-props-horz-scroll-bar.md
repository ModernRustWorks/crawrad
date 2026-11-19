---
id: props-props-horz-scroll-bar
kind: doc
target_file: src/props.rs
line_hint: 965
author: 
created: 
title: "Props::horz_scroll_bar"
tags: []
signature: "pub fn horz_scroll_bar (default : bool) -> PropDesc"
---

Horizontal scroll bar enabled (`HorzScrollBar`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::horz_scroll_bar(true);
assert_eq!(p.key, "horz_scroll_bar");
```
