---
id: code-action-offer
kind: doc
target_file: src/code_action.rs
line_hint: 57
author: 
created: 
title: "Offer"
tags: []
signature: "pub struct Offer"
---

The actions offered where the caret rests, and the request on the way.

# Examples

```rust
use crawrad_lsp::code_action::Offer;

let offer = Offer::default();
assert_eq!(offer.here, None);
```
