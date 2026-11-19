---
id: item-itemkind
kind: doc
target_file: src/item.rs
line_hint: 5
author: 
created: 
title: "ItemKind"
tags: []
signature: "pub enum ItemKind"
---

Category of a completion suggestion item.

Categoria do item sugerido na lista de completar.

# Examples

```rust
use crawrad_insight::item::ItemKind;

assert_eq!(ItemKind::Field.badge(), "\u{eb5f}");
```
