---
id: targets-shared-peca-cursor
kind: doc
target_file: src/targets/shared.rs
line_hint: 203
author: 
created: 
title: "PECA_CURSOR"
tags: []
signature: "pub const PECA_CURSOR : & str = include_str ! (\"../../../crawrad-component/src/pieces/cursor.rs\")"
---

O texto da peca `Cursor`.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_CURSOR;

assert!(PECA_CURSOR.contains("enum Cursor"));
```
