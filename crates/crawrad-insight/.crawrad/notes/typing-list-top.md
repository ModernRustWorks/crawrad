---
id: typing-list-top
kind: doc
target_file: src/typing.rs
line_hint: 135
author: 
created: 
title: "list_top"
tags: []
signature: "pub fn list_top (top : usize , selected : usize , moved_selection : bool , scrolled : isize , total : usize ,) -> usize"
---

Calculates visible top index when scrolling or moving list selection.

The first row the list shows. The keys move the selection and the
window follows it; the wheel moves the window (`scrolled`, in rows) and
leaves the selection where it is — as a modern editor's list.

# Examples

```rust
use crawrad_insight::typing::list_top;

assert_eq!(list_top(0, 0, false, 0, 20), 0);
```
