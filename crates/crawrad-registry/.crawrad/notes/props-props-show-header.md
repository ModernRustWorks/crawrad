---
id: props-props-show-header
kind: doc
target_file: src/props.rs
line_hint: 1992
author: 
created: 
title: "Props::show_header"
tags: []
signature: "pub fn show_header (default : bool) -> PropDesc"
---

Whether the card header (title and subtitle) is displayed (`ShowHeader`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::show_header(true);
assert_eq!(p.key, "show_header");
```
