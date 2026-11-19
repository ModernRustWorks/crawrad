---
id: props-props-binding-mode
kind: doc
target_file: src/props.rs
line_hint: 469
author: 
created: 
title: "Props::binding_mode"
tags: []
signature: "pub fn binding_mode () -> PropDesc"
---

Synchronization mode for the Inspector's Data category (#788 slice 3a).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::binding_mode();
assert_eq!(p.key, "binding.mode");
```
