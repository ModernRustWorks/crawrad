---
id: item-itemsugestao-key
kind: doc
target_file: src/item.rs
line_hint: 189
author: 
created: 
title: "ItemSugestao::key"
tags: []
signature: "pub fn key (& self) -> & str"
---

What the typed text is matched against: `filterText`, else the label.

# Examples

```rust
use crawrad_insight::item::{ItemKind, ItemSugestao};

let item = ItemSugestao::novo("test", ItemKind::Field);
assert_eq!(item.key(), "test");
```
