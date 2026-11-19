---
id: props-props-auto-scroll
kind: doc
target_file: src/props.rs
line_hint: 954
author: 
created: 
title: "Props::auto_scroll"
tags: []
signature: "pub fn auto_scroll (default : bool) -> PropDesc"
---

Automatic scrolling in container (`AutoScroll`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::auto_scroll(true);
assert_eq!(p.key, "auto_scroll");
```
