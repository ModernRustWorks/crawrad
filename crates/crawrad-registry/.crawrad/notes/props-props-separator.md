---
id: props-props-separator
kind: doc
target_file: src/props.rs
line_hint: 262
author: 
created: 
title: "Props::separator"
tags: []
signature: "pub fn separator (default : & str) -> PropDesc"
---

The separator string between breadcrumb segments (`Separator`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::separator("/");
assert_eq!(p.key, "separator");
```
