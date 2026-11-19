---
id: ide-app-layout-tab-item
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 81
author: 
created: 
title: "tab_item"
tags: []
signature: "pub fn tab_item (panel : Panel) -> DockItem"
---

A panel's tab item, named from the catalogue.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let item = tab_item(crawrad_actions::Panel::Inspector);
assert!(matches!(item, makepad_widgets::DockItem::Tab { .. }));
```
