---
id: options-savingcheck-label
kind: doc
target_file: src/options.rs
line_hint: 372
author: 
created: 
title: "SavingCheck::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

What the box says.

# Examples

```rust
use crawrad_project::options::SavingCheck;

assert_eq!(SavingCheck::SaveBeforeBuild.label(), "Save editor files before Build or Run");
```
