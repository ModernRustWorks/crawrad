---
id: props-props-char-case
kind: doc
target_file: src/props.rs
line_hint: 1304
author: 
created: 
title: "Props::char_case"
tags: []
signature: "pub fn char_case (default : & 'static str) -> PropDesc"
---

The case the typed text is held to (`CharCase`, #667).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::char_case("ecNormal");
assert_eq!(p.key, "char_case");
```
