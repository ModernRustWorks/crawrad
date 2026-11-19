---
id: chips-crawradchipsaction
kind: doc
target_file: src/chips.rs
line_hint: 126
author: 
created: 
title: "CrawradChipsAction"
tags: []
signature: "pub enum CrawradChipsAction"
---

What a [`CrawradChips`] reports.

# Examples

```rust
use crawrad_runtime_makepad::chips::CrawradChipsAction;

let action = CrawradChipsAction::Selected(0);
assert!(matches!(action, CrawradChipsAction::Selected(0)));
```
