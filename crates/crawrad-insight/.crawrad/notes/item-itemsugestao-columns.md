---
id: item-itemsugestao-columns
kind: doc
target_file: src/item.rs
line_hint: 194
author: 
created: 
title: "ItemSugestao::columns"
tags: []
signature: "pub fn columns (& self) -> Columns"
---

The row's four columns: kind icon glyph, label, origin qualifier, and signature.

# Examples

```rust
use crawrad_insight::item::{ItemKind, ItemSugestao};

let item = ItemSugestao::novo("test", ItemKind::Field);
assert_eq!(item.columns().label, "test");
```
