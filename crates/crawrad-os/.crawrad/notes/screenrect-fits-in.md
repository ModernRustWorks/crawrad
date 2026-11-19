---
id: screenrect-fits-in
kind: doc
target_file: src/lib.rs
line_hint: 88
author: 
created: 
title: "ScreenRect::fits_in"
tags: []
signature: "pub fn fits_in (& self , other : & ScreenRect) -> bool"
---

Returns true if this rectangle fits entirely inside `other`.

Does this rectangle fit entirely inside `other`?

# Examples

```rust
use crawrad_os::ScreenRect;

let a = ScreenRect { x: 10.0, y: 10.0, width: 20.0, height: 20.0 };
let b = ScreenRect { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
assert!(a.fits_in(&b));
```
