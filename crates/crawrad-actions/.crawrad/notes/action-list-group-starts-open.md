---
id: action-list-group-starts-open
kind: doc
target_file: src/action_list.rs
line_hint: 152
author: 
created: 
title: "group_starts_open"
tags: []
signature: "pub fn group_starts_open (cat : & str) -> bool"
---

Which categories are born OPEN in the standard classes window.

Two, and they are the egui ones: the uncategorized one and the edit one. It
is what the classic RAD IDE opens, and opening all of them in a list of thirty
classes would give a window that only scrolls.

# Examples

```rust
use crawrad_actions::action_list::group_starts_open;

assert!(group_starts_open("Edit"));
assert!(!group_starts_open("UnknownCategory"));
```
