---
id: list-selection-click
kind: doc
target_file: src/list_selection.rs
line_hint: 18
author: 
created: 
title: "click"
tags: []
signature: "pub fn click (selection : & mut Vec < usize > , focus : Option < usize > , row : usize , multi_select : bool , ctrl : bool , shift : bool ,)"
---

Updates row selection following a click.

What a click on `row` leaves in `selection`, given the row in `focus`
before the click. The caller then makes `row` the row in focus.

`selection` stays sorted and without repeats.

# Examples

```rust
use crawrad_runtime::list_selection::click;

let mut selection = Vec::new();
click(&mut selection, None, 2, false, false, false);
assert_eq!(selection, vec![2]);
```
