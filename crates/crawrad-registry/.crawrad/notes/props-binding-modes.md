---
id: props-binding-modes
kind: doc
target_file: src/props.rs
line_hint: 100
author: 
created: 
title: "BINDING_MODES"
tags: []
signature: "pub const BINDING_MODES : & [& str] = & [\"OneWay\" , \"TwoWay\" , \"OneWayToSource\" , \"OneTime\"]"
---

The four data binding synchronization modes (#788 slice 3a).

# Examples

```rust
use crawrad_registry::BINDING_MODES;

assert!(BINDING_MODES.contains(&"OneWay"));
```
