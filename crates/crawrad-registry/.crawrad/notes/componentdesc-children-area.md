---
id: componentdesc-children-area
kind: doc
target_file: src/lib.rs
line_hint: 643
author: 
created: 
title: "ComponentDesc::children_area"
tags: []
signature: "pub fn children_area (& self) -> bool"
---

Children appear **inside its area** on the canvas.

# Examples

```rust
use crawrad_registry::{action_list, panel};

assert!(panel().children_area());
assert!(!action_list().children_area());
```
