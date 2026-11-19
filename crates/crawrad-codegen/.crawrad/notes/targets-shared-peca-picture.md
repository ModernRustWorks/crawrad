---
id: targets-shared-peca-picture
kind: doc
target_file: src/targets/shared.rs
line_hint: 216
author: 
created: 
title: "PECA_PICTURE"
tags: []
signature: "pub const PECA_PICTURE : & str = include_str ! (\"../../../crawrad-component/src/pieces/picture.rs\")"
---

O texto da peca `Picture`.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_PICTURE;

assert!(PECA_PICTURE.contains("struct Picture"));
```
