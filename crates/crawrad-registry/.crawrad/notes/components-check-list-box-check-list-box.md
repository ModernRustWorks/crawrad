---
id: components-check-list-box-check-list-box
kind: doc
target_file: src/components/check_list_box.rs
line_hint: 4
author: 
created: 
title: "check_list_box"
tags: []
signature: "pub fn check_list_box () -> ComponentDesc"
---

A list of selectable items, each accompanied by a check box.

`CheckListBox`: a list of items, each with a check box;
any number can be checked.

# Examples

```rust
use crawrad_registry::check_list_box;

let desc = check_list_box();
assert_eq!(desc.class, "CheckListBox");
```
