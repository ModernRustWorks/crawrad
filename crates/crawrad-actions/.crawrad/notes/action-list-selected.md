---
id: action-list-selected
kind: doc
target_file: src/action_list.rs
line_hint: 95
author: 
created: 
title: "selected"
tags: []
signature: "pub fn selected (doc : & Document , list : & str , selection : & [String]) -> Option < String >"
---

The selected action, when the selection is indeed an action of this list.

`None` when nothing is selected, when the selection is not an `Action`, or
when it belongs to another list. All three cases yield the same on screen:
`Delete`, `Move Up` and `Move Down` disabled — and the house rule is that
what cannot be done STAYS, and stays disabled.

# Examples

```rust
use crawrad_actions::action_list::selected;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert_eq!(selected(&doc, "list1", &[]), None);
```
