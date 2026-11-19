---
id: list-view-crawradlistviewaction
kind: doc
target_file: src/list_view.rs
line_hint: 172
author: 
created: 
title: "CrawradListViewAction"
tags: []
signature: "pub enum CrawradListViewAction"
---

What a [`CrawradListView`] reports.

# Examples

```rust
use crawrad_runtime_makepad::list_view::CrawradListViewAction;

let action = CrawradListViewAction::Selected(0);
assert!(matches!(action, CrawradListViewAction::Selected(0)));
```
