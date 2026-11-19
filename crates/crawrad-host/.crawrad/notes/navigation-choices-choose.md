---
id: navigation-choices-choose
kind: doc
target_file: src/navigation.rs
line_hint: 117
author: 
created: 
title: "Choices::choose"
tags: []
signature: "pub fn choose (& mut self , i : usize) -> Option < (Place , Place) >"
---

The `i`-th place is chosen: it and the origin, once; the list closes.

# Examples

```rust
use crawrad_host::navigation::{Choices, Place};

let mut choices = Choices::default();
let target = Place::default();
let origin = Place::default();
choices.offer(vec![target.clone()], origin.clone());
assert_eq!(choices.choose(0), Some((target, origin)));
assert!(!choices.is_open());
```
