---
id: mode-bindingmode-is-continuous
kind: doc
target_file: src/mode.rs
line_hint: 45
author: 
created: 
title: "BindingMode::is_continuous"
tags: []
signature: "pub const fn is_continuous (& self) -> bool"
---

Returns true if the binding maintains active listeners for continuous updates.

# Examples

```rust
use crawrad_binding::BindingMode;

assert!(BindingMode::OneWay.is_continuous());
assert!(!BindingMode::OneTime.is_continuous());
```
