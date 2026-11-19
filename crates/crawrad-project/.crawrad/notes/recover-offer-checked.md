---
id: recover-offer-checked
kind: doc
target_file: src/recover.rs
line_hint: 216
author: 
created: 
title: "Offer::checked"
tags: []
signature: "pub fn checked (& self) -> Vec < PathBuf >"
---

The files of the checked rows.

# Examples

```rust
use crawrad_project::recover::Offer;

let offer = Offer::default();
assert!(offer.checked().is_empty());
```
