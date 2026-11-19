---
id: targets-shared-peca-font
kind: doc
target_file: src/targets/shared.rs
line_hint: 213
author: 
created: 
title: "PECA_FONT"
tags: []
signature: "pub const PECA_FONT : & str = include_str ! (\"../../../crawrad-component/src/pieces/font.rs\")"
---

O texto da peca `Font`.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_FONT;

assert!(PECA_FONT.contains("struct Font"));
```
