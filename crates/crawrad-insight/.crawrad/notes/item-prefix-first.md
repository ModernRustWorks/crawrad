---
id: item-prefix-first
kind: doc
target_file: src/item.rs
line_hint: 372
author: 
created: 
title: "prefix_first"
tags: []
signature: "pub fn prefix_first (items : & mut [ItemSugestao] , typed : & str)"
---

Names that start with what was typed come first; then by label.

# Examples

```rust
use crawrad_insight::item::{prefix_first, ItemKind, ItemSugestao};

let mut items = vec![
    ItemSugestao::novo("alpha", ItemKind::Field),
    ItemSugestao::novo("beta", ItemKind::Field),
];
prefix_first(&mut items, "be");
assert_eq!(items[0].rotulo, "beta");
```
