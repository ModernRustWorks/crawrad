---
id: props-props-flat
kind: doc
target_file: src/props.rs
line_hint: 1541
author: 
created: 
title: "Props::flat"
tags: []
signature: "pub fn flat (default : bool) -> PropDesc"
---

Flat appearance without border at rest (`Flat`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::flat(true);
assert_eq!(p.key, "flat");
```
