---
id: targets-shared-peca-modal-result
kind: doc
target_file: src/targets/shared.rs
line_hint: 206
author: 
created: 
title: "PECA_MODAL_RESULT"
tags: []
signature: "pub const PECA_MODAL_RESULT : & str = include_str ! (\"../../../crawrad-component/src/pieces/modal_result.rs\")"
---

O texto da peca `ModalResult`.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_MODAL_RESULT;

assert!(PECA_MODAL_RESULT.contains("enum ModalResult"));
```
