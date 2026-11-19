---
id: props-props-caption
kind: doc
target_file: src/props.rs
line_hint: 129
author: 
created: 
title: "Props::caption"
tags: []
signature: "pub fn caption (default : & str) -> PropDesc"
---

Textual caption / title (`Caption`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::caption("OK");
assert_eq!(p.key, "text");
```
