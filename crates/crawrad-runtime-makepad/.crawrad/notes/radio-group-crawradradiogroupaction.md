---
id: radio-group-crawradradiogroupaction
kind: doc
target_file: src/radio_group.rs
line_hint: 36
author: 
created: 
title: "CrawradRadioGroupAction"
tags: []
signature: "pub enum CrawradRadioGroupAction"
---

What a [`CrawradRadioGroup`] reports.

# Examples

```rust
use crawrad_runtime_makepad::radio_group::CrawradRadioGroupAction;

let action = CrawradRadioGroupAction::Picked(0);
assert!(matches!(action, CrawradRadioGroupAction::Picked(0)));
```
