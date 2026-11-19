---
id: check-list-crawradchecklistboxaction
kind: doc
target_file: src/check_list.rs
line_hint: 46
author: 
created: 
title: "CrawradCheckListBoxAction"
tags: []
signature: "pub enum CrawradCheckListBoxAction"
---

What a [`CrawradCheckListBox`] reports.

# Examples

```rust
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxAction;

let action = CrawradCheckListBoxAction::Toggled(1, true);
assert!(matches!(action, CrawradCheckListBoxAction::Toggled(1, true)));
```
