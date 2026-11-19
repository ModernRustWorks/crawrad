---
id: props-props-lines
kind: doc
target_file: src/props.rs
line_hint: 151
author: 
created: 
title: "Props::lines"
tags: []
signature: "pub fn lines (default : & str) -> PropDesc"
---

Text with line breaks (`Lines`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::lines("Line 1\nLine 2");
assert_eq!(p.key, "lines");
```
