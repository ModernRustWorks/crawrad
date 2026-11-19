---
id: props-props-show-arrows
kind: doc
target_file: src/props.rs
line_hint: 312
author: 
created: 
title: "Props::show_arrows"
tags: []
signature: "pub fn show_arrows (default : bool) -> PropDesc"
---

Whether a `Pagination` shows previous and next arrows (`ShowArrows`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::show_arrows(true);
assert_eq!(p.key, "show_arrows");
```
