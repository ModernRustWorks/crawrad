---
id: props-props-binding-field
kind: doc
target_file: src/props.rs
line_hint: 458
author: 
created: 
title: "Props::binding_field"
tags: []
signature: "pub fn binding_field () -> PropDesc"
---

Column or field name for the Inspector's Data category (#788 slice 3a).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::binding_field();
assert_eq!(p.key, "binding.field");
```
