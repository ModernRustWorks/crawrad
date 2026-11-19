---
id: mode-bindingmode-as-str
kind: doc
target_file: src/mode.rs
line_hint: 51
author: 
created: 
title: "BindingMode::as_str"
tags: []
signature: "pub const fn as_str (& self) -> & 'static str"
---

Returns the human-readable canonical display label.

# Examples

```rust
use crawrad_binding::BindingMode;

assert_eq!(BindingMode::TwoWay.as_str(), "TwoWay");
```
