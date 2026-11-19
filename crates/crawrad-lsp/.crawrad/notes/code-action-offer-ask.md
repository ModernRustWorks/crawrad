---
id: code-action-offer-ask
kind: doc
target_file: src/code_action.rs
line_hint: 74
author: 
created: 
title: "Offer::ask"
tags: []
signature: "pub fn ask (& mut self , place : Place , by_key : bool) -> u64"
---

A request for this place; the ticket its answer must carry.

# Examples

```rust
use crawrad_lsp::code_action::Offer;
use std::path::PathBuf;

let mut offer = Offer::default();
let place = (PathBuf::from("src/lib.rs"), 1, (0, 0));
let ticket = offer.ask(place.clone(), false);
assert!(offer.knows(&place));
assert_eq!(ticket, 1);
```
