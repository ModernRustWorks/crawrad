---
id: recover-offer
kind: doc
target_file: src/recover.rs
line_hint: 176
author: 
created: 
title: "Offer"
tags: []
signature: "pub struct Offer"
---

What the "Recover unsaved work" dialog shows.

What the "Recover unsaved work" dialog shows: the copies on offer, each
with its box — checked, it comes back with Recover selected.

# Examples

```rust
use crawrad_project::recover::Offer;

let offer = Offer::default();
assert!(offer.rows.is_empty());
```
