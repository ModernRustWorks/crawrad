---
id: props-props-bit-btn-kind
kind: doc
target_file: src/props.rs
line_hint: 1429
author: 
created: 
title: "Props::bit_btn_kind"
tags: []
signature: "pub fn bit_btn_kind (default : & 'static str) -> PropDesc"
---

Predefined button type with glyph (`Kind` of BitBtn).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::bit_btn_kind("bkOK");
assert_eq!(p.key, "kind");
```
