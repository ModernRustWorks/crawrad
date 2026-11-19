---
id: components-bit-btn-bit-btn
kind: doc
target_file: src/components/bit_btn.rs
line_hint: 4
author: 
created: 
title: "bit_btn"
tags: []
signature: "pub fn bit_btn () -> ComponentDesc"
---

Button with native glyph or icon support and standard modal results.

`BitBtn`: button with native glyph / icon support and classic standard modal results.

# Examples

```rust
use crawrad_registry::bit_btn;

let desc = bit_btn();
assert_eq!(desc.class, "BitBtn");
```
