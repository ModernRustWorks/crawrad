---
id: props-align-options
kind: doc
target_file: src/props.rs
line_hint: 72
author: 
created: 
title: "ALIGN_OPTIONS"
tags: []
signature: "pub const ALIGN_OPTIONS : & [& str] = & [\"None\" , \"Top\" , \"Bottom\" , \"Left\" , \"Right\" , \"Client\"]"
---

The default values of `Align`, in classic RAD IDE order.

# Examples

```rust
use crawrad_registry::ALIGN_OPTIONS;

assert!(ALIGN_OPTIONS.contains(&"Client"));
```
