---
id: recover-offer-files
kind: doc
target_file: src/recover.rs
line_hint: 222
author: 
created: 
title: "Offer::files"
tags: []
signature: "pub fn files (& self) -> Vec < PathBuf >"
---

The files of every row.

# Examples

```rust
use crawrad_project::recover::Offer;

let offer = Offer::default();
assert!(offer.files().is_empty());
```
