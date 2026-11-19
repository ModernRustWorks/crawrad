---
id: components-breadcrumb-breadcrumb
kind: doc
target_file: src/components/breadcrumb.rs
line_hint: 4
author: 
created: 
title: "breadcrumb"
tags: []
signature: "pub fn breadcrumb () -> ComponentDesc"
---

`Breadcrumb`: a navigation trail on one line (#500).

# Examples

```rust
use crawrad_registry::breadcrumb;

let desc = breadcrumb();
assert_eq!(desc.class, "Breadcrumb");
```
