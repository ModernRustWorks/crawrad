---
id: targets-shared-peca-color
kind: doc
target_file: src/targets/shared.rs
line_hint: 194
author: 
created: 
title: "PECA_COLOR"
tags: []
signature: "pub const PECA_COLOR : & str = include_str ! (\"../../../crawrad-component/src/pieces/color.rs\")"
---

O texto da peca `Color`.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_COLOR;

assert!(PECA_COLOR.contains("struct Color"));
```
