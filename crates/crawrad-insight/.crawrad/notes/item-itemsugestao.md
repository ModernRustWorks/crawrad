---
id: item-itemsugestao
kind: doc
target_file: src/item.rs
line_hint: 65
author: 
created: 
title: "ItemSugestao"
tags: []
signature: "pub struct ItemSugestao"
---

Completion suggestion item shown in the dropdown list.

Suggestion item in the completion list dropdown.

# Examples

```rust
use crawrad_insight::item::{ItemKind, ItemSugestao};

let item = ItemSugestao::novo("len", ItemKind::Method);
assert_eq!(item.rotulo, "len");
```
