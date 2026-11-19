---
id: mask-editmask-keeps-literals
kind: doc
target_file: src/mask.rs
line_hint: 190
author: 
created: 
title: "EditMask::keeps_literals"
tags: []
signature: "pub fn keeps_literals (& self) -> bool"
---

Whether the `Text` keeps the literals (`;1`, the default).

# Examples

```rust
use crawrad_ir::mask::EditMask;

assert!(EditMask::parse("00;1;_").keeps_literals());
assert!(!EditMask::parse("00;0;_").keeps_literals());
```
