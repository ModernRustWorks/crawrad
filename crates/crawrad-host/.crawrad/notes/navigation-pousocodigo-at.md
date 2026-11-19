---
id: navigation-pousocodigo-at
kind: doc
target_file: src/navigation.rs
line_hint: 308
author: 
created: 
title: "PousoCodigo::at"
tags: []
signature: "pub fn at (linha : usize , byte : usize) -> Self"
---

The caret on a 0-based line and byte, nothing selected.

# Examples

```rust
use crawrad_host::navigation::PousoCodigo;

let pouso = PousoCodigo::at(2, 4);
assert_eq!(pouso.linha, 2);
assert_eq!(pouso.start_byte, 4);
assert_eq!(pouso.end_byte, 4);
```
