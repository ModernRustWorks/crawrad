---
id: item-item-symbol
kind: doc
target_file: src/item.rs
line_hint: 115
author: 
created: 
title: "Item::symbol"
tags: []
signature: "pub fn symbol < 'a > (& self , symbols : & 'a [String]) -> Option < & 'a str >"
---

Returns this item's icon symbol from a slice of symbols.

This item's icon in `symbols` (the linked ImageList's), if its
`image_index` points at one.

# Examples

```rust
use crawrad_ir::Item;

let mut item = Item::new("File");
item.image_index = 0;
let symbols = vec!["folder".to_string(), "document".to_string()];
assert_eq!(item.symbol(&symbols), Some("folder"));
```
