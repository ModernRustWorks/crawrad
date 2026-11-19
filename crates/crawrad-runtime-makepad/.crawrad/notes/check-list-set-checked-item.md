---
id: check-list-set-checked-item
kind: doc
target_file: src/check_list.rs
line_hint: 266
author: 
created: 
title: "set_checked_item"
tags: []
signature: "pub fn set_checked_item (items : & [String] , checked : & mut Vec < String > , index : usize , on : bool)"
---

Updates `checked` when row `index` is toggled `on` or off.

Adds or removes `items[index]` without duplicates; index past the end does nothing.

# Examples

```rust
use crawrad_runtime_makepad::check_list::set_checked_item;

let items = vec!["A".to_string(), "B".to_string()];
let mut checked = Vec::new();
set_checked_item(&items, &mut checked, 0, true);
assert_eq!(checked, vec!["A".to_string()]);
```
