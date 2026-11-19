---
id: action-list-standardrow
kind: doc
target_file: src/action_list.rs
line_hint: 102
author: 
created: 
title: "StandardRow"
tags: []
signature: "pub enum StandardRow"
---

A row of the **Standard Action Classes** window, already flattened.

# Examples

```rust
use crawrad_actions::action_list::StandardRow;

let row = StandardRow::Group { name: "Edit", open: true };
assert!(matches!(row, StandardRow::Group { open: true, .. }));
```
