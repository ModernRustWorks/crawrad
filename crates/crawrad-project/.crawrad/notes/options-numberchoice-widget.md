---
id: options-numberchoice-widget
kind: doc
target_file: src/options.rs
line_hint: 224
author: 
created: 
title: "NumberChoice::widget"
tags: []
signature: "pub fn widget (self) -> & 'static str"
---

Its drop down, by the DSL name.

# Examples

```rust
use crawrad_project::options::NumberChoice;

assert_eq!(NumberChoice::Limit.widget(), "opt_history_limit");
```
