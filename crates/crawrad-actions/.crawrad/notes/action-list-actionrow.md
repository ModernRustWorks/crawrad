---
id: action-list-actionrow
kind: doc
target_file: src/action_list.rs
line_hint: 55
author: 
created: 
title: "ActionRow"
tags: []
signature: "pub struct ActionRow"
---

An action in the right column.

# Examples

```rust
use crawrad_actions::action_list::ActionRow;

let row = ActionRow {
    id: "act1".to_string(),
    label: "act1  ·  New".to_string(),
};
assert_eq!(row.id, "act1");
```
