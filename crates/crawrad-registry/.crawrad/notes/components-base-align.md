---
id: components-base-align
kind: doc
target_file: src/components/base.rs
line_hint: 8
author: 
created: 
title: "ALIGN"
tags: []
signature: "pub const ALIGN : & [& str] = & [\"None\" , \"Top\" , \"Bottom\" , \"Left\" , \"Right\" , \"Client\"]"
---

The values of `Align`, in classic RAD IDE order.

# Examples

```rust
use crawrad_registry::ALIGN;

assert!(ALIGN.contains(&"Client"));
```
