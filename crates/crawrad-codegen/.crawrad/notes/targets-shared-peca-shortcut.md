---
id: targets-shared-peca-shortcut
kind: doc
target_file: src/targets/shared.rs
line_hint: 210
author: 
created: 
title: "PECA_SHORTCUT"
tags: []
signature: "pub const PECA_SHORTCUT : & str = include_str ! (\"../../../crawrad-component/src/pieces/shortcut.rs\")"
---

O texto da peca `Shortcut`.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_SHORTCUT;

assert!(PECA_SHORTCUT.contains("struct Shortcut"));
```
