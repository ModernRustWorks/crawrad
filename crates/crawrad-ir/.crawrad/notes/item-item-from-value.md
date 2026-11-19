---
id: item-item-from-value
kind: doc
target_file: src/item.rs
line_hint: 59
author: 
created: 
title: "Item::from_value"
tags: []
signature: "pub fn from_value (v : & Value) -> Option < Self >"
---

Reads an item from a [`Value::Map`].

Reads an item from a `Value::Map`. Returns `None` for anything else —
a malformed collection does not become a half-item.

# Examples

```rust
use crawrad_ir::{Item, Value};

let item = Item::new("Node");
let val = item.to_value();
assert_eq!(Item::from_value(&val), Some(item));
```
