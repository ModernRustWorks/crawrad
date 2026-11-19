---
id: mode-bindingmode-propagates-target-to-source
kind: doc
target_file: src/mode.rs
line_hint: 39
author: 
created: 
title: "BindingMode::propagates_target_to_source"
tags: []
signature: "pub const fn propagates_target_to_source (& self) -> bool"
---

Returns true if changes in the target should flow to the source.

# Examples

```rust
use crawrad_binding::BindingMode;

assert!(BindingMode::TwoWay.propagates_target_to_source());
assert!(!BindingMode::OneWay.propagates_target_to_source());
```
