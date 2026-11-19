---
id: string-grid-crawradstringgridaction
kind: doc
target_file: src/string_grid.rs
line_hint: 41
author: 
created: 
title: "CrawradStringGridAction"
tags: []
signature: "pub enum CrawradStringGridAction"
---

What a [`CrawradStringGrid`] reports.

# Examples

```rust
use crawrad_runtime_makepad::string_grid::CrawradStringGridAction;

let action = CrawradStringGridAction::Clicked;
assert!(matches!(action, CrawradStringGridAction::Clicked));
```
