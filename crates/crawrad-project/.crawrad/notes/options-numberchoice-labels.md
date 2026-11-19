---
id: options-numberchoice-labels
kind: doc
target_file: src/options.rs
line_hint: 249
author: 
created: 
title: "NumberChoice::labels"
tags: []
signature: "pub fn labels (self) -> Vec < String >"
---

What the drop down shows, one per value.

# Examples

```rust
use crawrad_project::options::NumberChoice;

assert_eq!(NumberChoice::Limit.labels()[0], "1 copies");
```
