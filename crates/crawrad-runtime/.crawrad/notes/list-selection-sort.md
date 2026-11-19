---
id: list-selection-sort
kind: doc
target_file: src/list_selection.rs
line_hint: 65
author: 
created: 
title: "sort"
tags: []
signature: "pub fn sort (items : & mut [String] , selected : & mut Option < usize > , selection : & mut [usize])"
---

Sorts list items and adjusts active selection indices.

Puts `items` in `Sorted` order, with `selected` and `selection` following
their rows: a row the code adds lands in its place, and the row the user
picked stays picked. Does nothing, and allocates nothing, when the list is
already in order — the generated code calls it before every draw.

# Examples

```rust
use crawrad_runtime::list_selection::sort;

let mut items = vec!["banana".to_string(), "apple".to_string()];
let mut selected = Some(0);
let mut selection = vec![0];
sort(&mut items, &mut selected, &mut selection);
assert_eq!(items, vec!["apple", "banana"]);
assert_eq!(selected, Some(1));
assert_eq!(selection, vec![1]);
```
