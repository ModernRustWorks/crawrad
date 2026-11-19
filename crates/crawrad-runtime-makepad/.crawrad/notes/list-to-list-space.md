---
id: list-to-list-space
kind: doc
target_file: src/list.rs
line_hint: 317
author: 
created: 
title: "to_list_space"
tags: []
signature: "pub fn to_list_space (rows : & [(usize , (String , Rect))] , top : DVec2) -> Vec < (String , Rect) >"
---

Translates drawn row bounds into the coordinate space of the list.

Puts drawn rows (index, text, rect in the draw pass) into the list's own
coordinates, `top` being where the list is: the first drawn row sits at
`top` plus its index times the row height (#604).

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_runtime_makepad::list::to_list_space;

let rows = to_list_space(&[], dvec2(0.0, 0.0));
assert!(rows.is_empty());
```
