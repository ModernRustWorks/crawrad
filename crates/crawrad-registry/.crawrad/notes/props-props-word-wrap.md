---
id: props-props-word-wrap
kind: doc
target_file: src/props.rs
line_hint: 1248
author: 
created: 
title: "Props::word_wrap"
tags: []
signature: "pub fn word_wrap (default : bool) -> PropDesc"
---

A long line breaks at the edge instead of running past it (`WordWrap`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::word_wrap(true);
assert_eq!(p.key, "word_wrap");
```
