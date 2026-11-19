---
id: mode-bindingmode
kind: doc
target_file: src/mode.rs
line_hint: 10
author: 
created: 
title: "BindingMode"
tags: []
signature: "pub enum BindingMode"
---

The synchronization direction between a data source and a UI target.

# Examples

```rust
use crawrad_binding::BindingMode;

let mode = BindingMode::TwoWay;
assert!(mode.propagates_source_to_target());
```
