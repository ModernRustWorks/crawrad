---
id: props-props-picture
kind: doc
target_file: src/props.rs
line_hint: 598
author: 
created: 
title: "Props::picture"
tags: []
signature: "pub fn picture (default : impl Into < crawrad_ir :: Picture >) -> PropDesc"
---

Picture graphic asset (`Picture` / `TPicture`).

# Examples

```rust
use crawrad_ir::Picture;
use crawrad_registry::Props;

let p = Props::picture(Picture::empty());
assert_eq!(p.key, "picture");
```
