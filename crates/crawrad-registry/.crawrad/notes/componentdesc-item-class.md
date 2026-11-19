---
id: componentdesc-item-class
kind: doc
target_file: src/lib.rs
line_hint: 627
author: 
created: 
title: "ComponentDesc::item_class"
tags: []
signature: "pub fn item_class (& self) -> Option < & 'static str >"
---

The class of the items, when this component is a collection.

# Examples

```rust
use crawrad_registry::action_list;

assert_eq!(action_list().item_class(), Some("Action"));
```
