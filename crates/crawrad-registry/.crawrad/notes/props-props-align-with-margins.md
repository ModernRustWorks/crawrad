---
id: props-props-align-with-margins
kind: doc
target_file: src/props.rs
line_hint: 389
author: 
created: 
title: "Props::align_with_margins"
tags: []
signature: "pub fn align_with_margins (default : bool) -> PropDesc"
---

Enables margin respect when `Align` is active (`AlignWithMargins`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::align_with_margins(true);
assert_eq!(p.key, "align_with_margins");
```
