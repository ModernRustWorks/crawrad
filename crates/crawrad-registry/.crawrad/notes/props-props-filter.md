---
id: props-props-filter
kind: doc
target_file: src/props.rs
line_hint: 1664
author: 
created: 
title: "Props::filter"
tags: []
signature: "pub fn filter (default : impl Into < String >) -> PropDesc"
---

File extension filter (`Filter`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::filter("Text files (*.txt)|*.txt");
assert_eq!(p.key, "filter");
```
