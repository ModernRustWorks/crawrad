---
id: mask-editmask-is-empty
kind: doc
target_file: src/mask.rs
line_hint: 180
author: 
created: 
title: "EditMask::is_empty"
tags: []
signature: "pub fn is_empty (& self) -> bool"
---

No position at all: the text passes untouched.

# Examples

```rust
use crawrad_ir::mask::EditMask;

assert!(EditMask::default().is_empty());
assert!(!EditMask::parse("00").is_empty());
```
