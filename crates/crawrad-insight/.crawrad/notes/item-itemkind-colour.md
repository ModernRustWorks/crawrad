---
id: item-itemkind-colour
kind: doc
target_file: src/item.rs
line_hint: 54
author: 
created: 
title: "ItemKind::colour"
tags: []
signature: "pub fn colour (& self) -> [f32 ; 4]"
---

The icon's colour, `[r, g, b, a]`.

# Examples

```rust
use crawrad_insight::item::ItemKind;

let rgba = ItemKind::Field.colour();
assert_eq!(rgba[3], 1.0);
```
