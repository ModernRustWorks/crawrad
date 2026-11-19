---
id: ide-app-layout-lone-group
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 90
author: 
created: 
title: "lone_group"
tags: []
signature: "pub fn lone_group (panel : Panel) -> DockItem"
---

A closable tab group holding only `panel`.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let group = lone_group(crawrad_actions::Panel::Inspector);
assert!(matches!(group, makepad_widgets::DockItem::Tabs { .. }));
```
