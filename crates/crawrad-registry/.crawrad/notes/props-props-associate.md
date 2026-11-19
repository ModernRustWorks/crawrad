---
id: props-props-associate
kind: doc
target_file: src/props.rs
line_hint: 2048
author: 
created: 
title: "Props::associate"
tags: []
signature: "pub fn associate () -> PropDesc"
---

Control anchoring this `Badge` at its top-right corner (`Associate`, #500).

Associated control whose top-right corner anchors this `Badge` (`Associate`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::associate();
assert_eq!(p.key, "associate");
```
