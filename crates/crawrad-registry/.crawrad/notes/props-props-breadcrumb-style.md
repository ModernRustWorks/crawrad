---
id: props-props-breadcrumb-style
kind: doc
target_file: src/props.rs
line_hint: 2116
author: 
created: 
title: "Props::breadcrumb_style"
tags: []
signature: "pub fn breadcrumb_style (default : & str) -> PropDesc"
---

Visual style of a `Breadcrumb` (`Style`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::breadcrumb_style("bsText");
assert_eq!(p.key, "style");
```
