---
id: viewcenter-toggle-code
kind: doc
target_file: src/lib.rs
line_hint: 88
author: 
created: 
title: "ViewCenter::toggle_code"
tags: []
signature: "pub fn toggle_code (self) -> Self"
---

`F12`: form ⇄ unit, there and back.

# Examples

```rust
use crawrad_host::ViewCenter;

assert_eq!(ViewCenter::Design.toggle_code(), ViewCenter::Code);
```
