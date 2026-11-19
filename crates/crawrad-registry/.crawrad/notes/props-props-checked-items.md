---
id: props-props-checked-items
kind: doc
target_file: src/props.rs
line_hint: 700
author: 
created: 
title: "Props::checked_items"
tags: []
signature: "pub fn checked_items () -> PropDesc"
---

Checked items texts for `CheckListBox` (`CheckedItems`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::checked_items();
assert_eq!(p.key, "checked_items");
```
