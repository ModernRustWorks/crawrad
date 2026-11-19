---
id: items-editor-itemrow
kind: doc
target_file: src/items_editor.rs
line_hint: 37
author: 
created: 
title: "ItemRow"
tags: []
signature: "pub struct ItemRow"
---

A tree row flattened for drawing, in reading order.

It exists because BOTH shells need the same traversal — the stack visiting in
order carrying depth along — and because Makepad's `PortalList` only knows
how to draw a flat list: it asks for item index `i`, and someone had to have
decided beforehand what item `i` is.

# Examples

```rust
use crawrad_actions::items_editor::ItemRow;

let row = ItemRow {
    text: "Root".into(),
    level: 0,
    path: vec![0],
};
assert_eq!(row.level, 0);
```
