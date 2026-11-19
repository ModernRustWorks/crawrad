---
id: options-numberchoice-values
kind: doc
target_file: src/options.rs
line_hint: 239
author: 
created: 
title: "NumberChoice::values"
tags: []
signature: "pub fn values (self) -> & 'static [u32]"
---

The values offered.

# Examples

```rust
use crawrad_project::options::NumberChoice;

assert!(NumberChoice::Limit.values().contains(&10));
```
