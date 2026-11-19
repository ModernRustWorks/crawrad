---
id: manifest-children
kind: doc
target_file: src/manifest.rs
line_hint: 767
author: 
created: 
title: "Children"
tags: []
signature: "pub enum Children"
---

Whether the component accepts children, and where they appear.

# Examples

```rust
use crawrad_package::manifest::Children;

let c = Children::None;
assert_eq!(c, Children::None);
```
