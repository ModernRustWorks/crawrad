---
id: recover-offer-check
kind: doc
target_file: src/recover.rs
line_hint: 209
author: 
created: 
title: "Offer::check"
tags: []
signature: "pub fn check (& mut self , i : usize , on : bool)"
---

Sets the box of row `i`; one out of the list changes nothing.

# Examples

```rust
use crawrad_project::recover::Offer;

let mut offer = Offer::default();
offer.check(0, true);
assert!(offer.rows.is_empty());
```
