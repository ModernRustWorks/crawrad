---
id: viewcenter-as-str
kind: doc
target_file: src/lib.rs
line_hint: 69
author: 
created: 
title: "ViewCenter::as_str"
tags: []
signature: "pub fn as_str (self) -> & 'static str"
---

Converts a [`ViewCenter`] to its canonical lowercase identifier.

# Examples

```rust
use crawrad_host::ViewCenter;

assert_eq!(ViewCenter::Design.as_str(), "design");
```
