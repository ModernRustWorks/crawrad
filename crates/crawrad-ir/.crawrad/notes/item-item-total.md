---
id: item-item-total
kind: doc
target_file: src/item.rs
line_hint: 147
author: 
created: 
title: "Item::total"
tags: []
signature: "pub fn total (items : & [Self]) -> usize"
---

How many items are in the entire tree, counting descendants.

# Examples

```rust
use crawrad_ir::Item;

let items = vec![
    Item::with_children("Root", vec![Item::new("Child 1"), Item::new("Child 2")]),
];
assert_eq!(Item::total(&items), 3);
```
