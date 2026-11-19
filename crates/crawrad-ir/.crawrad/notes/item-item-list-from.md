---
id: item-item-list-from
kind: doc
target_file: src/item.rs
line_hint: 103
author: 
created: 
title: "Item::list_from"
tags: []
signature: "pub fn list_from (v : & Value) -> Vec < Self >"
---

The entire collection, from a `Value::List`.

# Examples

```rust
use crawrad_ir::{Item, Value};

let list = Value::List(vec![Item::new("A").to_value()]);
let items = Item::list_from(&list);
assert_eq!(items.len(), 1);
assert_eq!(items[0].text, "A");
```
