---
id: props-props-transparent
kind: doc
target_file: src/props.rs
line_hint: 1418
author: 
created: 
title: "Props::transparent"
tags: []
signature: "pub fn transparent (default : bool) -> PropDesc"
---

Image background transparency (`Transparent`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::transparent(true);
assert_eq!(p.key, "transparent");
```
