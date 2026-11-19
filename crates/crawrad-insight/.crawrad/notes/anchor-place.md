---
id: anchor-place
kind: doc
target_file: src/anchor.rs
line_hint: 19
author: 
created: 
title: "place"
tags: []
signature: "pub fn place (caret : [f64 ; 2] , typed : usize , cell : [f64 ; 2] , container : [f64 ; 4] , size : [f64 ; 2] ,) -> Anchor"
---

Computes completion list placement within container bounds.

`caret` and `container` (x, y, w, h) in window pixels, `cell` the text's
cell (w, h), `typed` the chars of the prefix already typed, `size` the
list's (w, h). The list stays inside the container: pulled left only
when it would leave its right side, opened above the line when it would
leave its bottom.

# Examples

```rust
use crawrad_insight::anchor::place;

let a = place([50.0, 100.0], 0, [8.0, 16.0], [0.0, 0.0, 800.0, 600.0], [200.0, 150.0]);
assert_eq!(a.word, [50.0, 100.0]);
```
