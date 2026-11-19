---
id: multi-select-toggle-selected
kind: doc
target_file: src/multi_select.rs
line_hint: 135
author: 
created: 
title: "toggle_selected"
tags: []
signature: "pub fn toggle_selected (items : & [String] , selected : & mut Vec < String > , index : usize)"
---

Toggles whether `items[index]` is present in `selected`.

Toggles whether `items[index]` is present in `selected`, keeping `selected`
in `items` order.

# Examples

```rust
use crawrad_runtime_makepad::multi_select::toggle_selected;

let items = vec!["A".to_string(), "B".to_string()];
let mut selected = Vec::new();
toggle_selected(&items, &mut selected, 0);
assert_eq!(selected, vec!["A".to_string()]);
```
