---
id: item-item
kind: doc
target_file: src/item.rs
line_hint: 20
author: 
created: 
title: "Item"
tags: []
signature: "pub struct Item"
---

An item in a tree collection.

# Examples

```rust
use crawrad_ir::Item;

let item = Item::new("Root");
assert_eq!(item.text, "Root");
assert!(item.children.is_empty());
```
