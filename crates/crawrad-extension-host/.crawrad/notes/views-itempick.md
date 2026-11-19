---
id: views-itempick
kind: doc
target_file: src/views.rs
line_hint: 152
author: 
created: 
title: "ItemPick"
tags: []
signature: "pub struct ItemPick"
---

Result of a user click on a view item.

What a click on an item asks of the window: tell its extension, and
open the place when it has one.

# Examples

```rust
use crawrad_extension_host::views::ItemPick;

let pick = ItemPick {
    extension: "ext1".into(),
    view: "v1".into(),
    item: "i1".into(),
    location: None,
};
assert_eq!(pick.item, "i1");
```
