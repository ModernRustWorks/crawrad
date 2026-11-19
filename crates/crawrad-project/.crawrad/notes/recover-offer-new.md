---
id: recover-offer-new
kind: doc
target_file: src/recover.rs
line_hint: 194
author: 
created: 
title: "Offer::new"
tags: []
signature: "pub fn new (found : & [Pending] , now : SystemTime , offset : i64) -> Self"
---

Every copy found, checked: bringing back is what the user came for.

# Examples

```rust
use std::time::SystemTime;
use crawrad_project::recover::Offer;

let offer = Offer::new(&[], SystemTime::UNIX_EPOCH, 0);
assert!(offer.rows.is_empty());
```
