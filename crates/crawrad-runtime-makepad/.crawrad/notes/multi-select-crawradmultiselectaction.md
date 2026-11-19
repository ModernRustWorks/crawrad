---
id: multi-select-crawradmultiselectaction
kind: doc
target_file: src/multi_select.rs
line_hint: 103
author: 
created: 
title: "CrawradMultiSelectAction"
tags: []
signature: "pub enum CrawradMultiSelectAction"
---

What [`CrawradMultiSelect`] reports.

# Examples

```rust
use crawrad_runtime_makepad::multi_select::CrawradMultiSelectAction;

let action = CrawradMultiSelectAction::Changed;
assert!(matches!(action, CrawradMultiSelectAction::Changed));
```
