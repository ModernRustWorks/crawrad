---
id: list-selection-settle
kind: doc
target_file: src/list_selection.rs
line_hint: 42
author: 
created: 
title: "settle"
tags: []
signature: "pub fn settle (selection : & mut Vec < usize > , selected : Option < usize > , len : usize , multi_select : bool)"
---

Synchronizes list selection bounds before rendering.

Puts `selection` back in step before the list is drawn: with
`MultiSelect` off it is `selected` (code that sets `selected` moves the
mark with it), and rows past the end of the list leave it — the code that
removed them left the indices behind.

# Examples

```rust
use crawrad_runtime::list_selection::settle;

let mut selection = vec![0, 5];
settle(&mut selection, Some(0), 3, true);
assert_eq!(selection, vec![0]);
```
