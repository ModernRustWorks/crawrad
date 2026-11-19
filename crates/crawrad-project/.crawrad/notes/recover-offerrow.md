---
id: recover-offerrow
kind: doc
target_file: src/recover.rs
line_hint: 182
author: 
created: 
title: "OfferRow"
tags: []
signature: "pub struct OfferRow"
---

One line of the dialog.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::recover::OfferRow;

let row = OfferRow {
    file: PathBuf::from("src/main.rs"),
    copy: "12:00".into(),
    disk: "11:00".into(),
    checked: true,
};
assert!(row.checked);
```
