---
id: list-crawradlistboxaction
kind: doc
target_file: src/list.rs
line_hint: 63
author: 
created: 
title: "CrawradListBoxAction"
tags: []
signature: "pub enum CrawradListBoxAction"
---

What a [`CrawradListBox`] reports.

# Examples

```rust
use crawrad_runtime_makepad::list::CrawradListBoxAction;

let action = CrawradListBoxAction::Selected(0);
assert!(matches!(action, CrawradListBoxAction::Selected(0)));
```
