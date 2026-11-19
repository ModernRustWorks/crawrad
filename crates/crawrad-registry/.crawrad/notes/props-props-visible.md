---
id: props-props-visible
kind: doc
target_file: src/props.rs
line_hint: 186
author: 
created: 
title: "Props::visible"
tags: []
signature: "pub fn visible (default : bool) -> PropDesc"
---

Component visibility on screen (`Visible`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::visible(true);
assert_eq!(p.key, "visible");
```
