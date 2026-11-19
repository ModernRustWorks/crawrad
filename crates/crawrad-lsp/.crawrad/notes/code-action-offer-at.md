---
id: code-action-offer-at
kind: doc
target_file: src/code_action.rs
line_hint: 112
author: 
created: 
title: "Offer::at"
tags: []
signature: "pub fn at (& self , place : & Place) -> Option < & [CodeAction] >"
---

The actions for this place, when its answer is in.

# Examples

```rust
use crawrad_lsp::code_action::Offer;
use std::path::PathBuf;

let offer = Offer::default();
let place = (PathBuf::from("src/lib.rs"), 1, (0, 0));
assert_eq!(offer.at(&place), None);
```
