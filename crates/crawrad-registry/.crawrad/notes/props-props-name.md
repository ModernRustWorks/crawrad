---
id: props-props-name
kind: doc
target_file: src/props.rs
line_hint: 107
author: 
created: 
title: "Props::name"
tags: []
signature: "pub fn name () -> PropDesc"
---

Component identifier (`Name`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::name();
assert_eq!(p.key, "name");
```
