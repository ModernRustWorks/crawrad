---
id: options-savingcheck-set
kind: doc
target_file: src/options.rs
line_hint: 403
author: 
created: 
title: "SavingCheck::set"
tags: []
signature: "pub fn set (self , p : & mut crate :: workspace :: Preferences , on : bool)"
---

Stores the box.

# Examples

```rust
use crawrad_project::options::SavingCheck;
use crawrad_project::workspace::Preferences;

let mut p = Preferences::default();
SavingCheck::SaveBeforeBuild.set(&mut p, true);
assert!(p.save_before_build);
```
