---
id: props-props-subtitle
kind: doc
target_file: src/props.rs
line_hint: 1959
author: 
created: 
title: "Props::subtitle"
tags: []
signature: "pub fn subtitle (default : & str) -> PropDesc"
---

A supporting secondary title under `Caption` (`Subtitle`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::subtitle("Details");
assert_eq!(p.key, "subtitle");
```
