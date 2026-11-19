---
id: props-props-proportional
kind: doc
target_file: src/props.rs
line_hint: 1385
author: 
created: 
title: "Props::proportional"
tags: []
signature: "pub fn proportional (default : bool) -> PropDesc"
---

Preserves original aspect ratio when stretching image (`Proportional`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::proportional(true);
assert_eq!(p.key, "proportional");
```
