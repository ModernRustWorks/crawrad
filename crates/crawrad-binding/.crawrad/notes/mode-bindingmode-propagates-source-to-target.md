---
id: mode-bindingmode-propagates-source-to-target
kind: doc
target_file: src/mode.rs
line_hint: 33
author: 
created: 
title: "BindingMode::propagates_source_to_target"
tags: []
signature: "pub const fn propagates_source_to_target (& self) -> bool"
---

Returns true if changes in the source should flow to the target.

# Examples

```rust
use crawrad_binding::BindingMode;

assert!(BindingMode::OneWay.propagates_source_to_target());
assert!(!BindingMode::OneWayToSource.propagates_source_to_target());
```
