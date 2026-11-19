---
id: components-page-control-tabwindow
kind: doc
target_file: src/components/page_control.rs
line_hint: 17
author: 
created: 
title: "TabWindow"
tags: []
signature: "pub struct TabWindow"
---

Which tabs of a strip are on screen (#13).

# Examples

```rust
use crawrad_registry::TabWindow;

let win = TabWindow { first: 0, count: 3, arrows: false };
assert_eq!(win.count, 3);
```
