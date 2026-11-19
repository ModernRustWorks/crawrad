---
id: action-list-is-flat-collection
kind: doc
target_file: src/action_list.rs
line_hint: 158
author: 
created: 
title: "is_flat_collection"
tags: []
signature: "pub fn is_flat_collection (registry : & Registry , class : & str) -> bool"
---

Tells whether the collection dialog edits a class's typed collection.

Its items nest nothing, or nest a collection of ANOTHER class (a `GridColumn`'s
`StyleRules`, opened one level down). A tree — items holding items of their own
class, like `MenuItem` — is not: it has its own designer.

# Examples

```rust
use crawrad_actions::action_list::is_flat_collection;
use crawrad_registry::Registry;

let reg = Registry::builtin();
assert!(is_flat_collection(&reg, "Grid"));
assert!(is_flat_collection(&reg, "GridColumn"));
assert!(!is_flat_collection(&reg, "MainMenu"));
assert!(!is_flat_collection(&reg, "Button"));
```
