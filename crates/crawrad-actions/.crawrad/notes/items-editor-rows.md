---
id: items-editor-rows
kind: doc
target_file: src/items_editor.rs
line_hint: 48
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (items : & [Item]) -> Vec < ItemRow >"
---

The entire tree in reading order.

# Examples

```rust
use crawrad_actions::items_editor::rows;
use crawrad_ir::Item;

let items = vec![Item::new("First"), Item::new("Second")];
let flattened = rows(&items);
assert_eq!(flattened.len(), 2);
```
