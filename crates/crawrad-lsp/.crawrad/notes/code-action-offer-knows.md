---
id: code-action-offer-knows
kind: doc
target_file: src/code_action.rs
line_hint: 68
author: 
created: 
title: "Offer::knows"
tags: []
signature: "pub fn knows (& self , place : & Place) -> bool"
---

Is this place already answered or asked? Then it is not asked again.

# Examples

```rust
use crawrad_lsp::code_action::Offer;
use std::path::PathBuf;

let offer = Offer::default();
let place = (PathBuf::from("src/lib.rs"), 1, (0, 0));
assert!(!offer.knows(&place));
```
