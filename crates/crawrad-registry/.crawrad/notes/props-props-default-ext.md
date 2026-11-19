---
id: props-props-default-ext
kind: doc
target_file: src/props.rs
line_hint: 1697
author: 
created: 
title: "Props::default_ext"
tags: []
signature: "pub fn default_ext (default : impl Into < String >) -> PropDesc"
---

Default file extension (`DefaultExt`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::default_ext("txt");
assert_eq!(p.key, "default_ext");
```
