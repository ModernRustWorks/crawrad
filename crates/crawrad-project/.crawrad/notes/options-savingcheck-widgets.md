---
id: options-savingcheck-widgets
kind: doc
target_file: src/options.rs
line_hint: 382
author: 
created: 
title: "SavingCheck::widgets"
tags: []
signature: "pub fn widgets (self) -> & 'static [& 'static str]"
---

The widgets that show it, by their DSL names.

# Examples

```rust
use crawrad_project::options::SavingCheck;

assert!(SavingCheck::SaveBeforeBuild.widgets().contains(&"opt_save_build"));
```
