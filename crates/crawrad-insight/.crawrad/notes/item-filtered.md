---
id: item-filtered
kind: doc
target_file: src/item.rs
line_hint: 336
author: 
created: 
title: "filtered"
tags: []
signature: "pub fn filtered (all : & [ItemSugestao] , typed : & str) -> Vec < ItemSugestao >"
---

Filters and ranks suggestion items based on typed input.

`all`, narrowed and ordered by what is typed (`crawrad_insight::typing`).
While rust-analyzer has not answered, its `rust-analyzer…` row stays when
nothing else matches.

# Examples

```rust
use crawrad_insight::item::{filtered, ItemKind, ItemSugestao};

let all = vec![ItemSugestao::novo("hello", ItemKind::Field)];
assert_eq!(filtered(&all, "he").len(), 1);
```
