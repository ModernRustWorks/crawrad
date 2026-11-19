---
id: props-props-tab-order
kind: doc
target_file: src/props.rs
line_hint: 400
author: 
created: 
title: "Props::tab_order"
tags: []
signature: "pub fn tab_order (default : i64) -> PropDesc"
---

Focus order in keyboard navigation (`TabOrder`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::tab_order(0);
assert_eq!(p.key, "tab_order");
```
