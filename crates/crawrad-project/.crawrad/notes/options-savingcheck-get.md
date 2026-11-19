---
id: options-savingcheck-get
kind: doc
target_file: src/options.rs
line_hint: 392
author: 
created: 
title: "SavingCheck::get"
tags: []
signature: "pub fn get (self , p : & crate :: workspace :: Preferences) -> bool"
---

Whether the box is checked.

# Examples

```rust
use crawrad_project::options::SavingCheck;
use crawrad_project::workspace::Preferences;

let p = Preferences::default();
assert_eq!(SavingCheck::SaveBeforeBuild.get(&p), p.save_before_build);
```
