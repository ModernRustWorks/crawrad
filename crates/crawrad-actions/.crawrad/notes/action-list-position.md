---
id: action-list-position
kind: doc
target_file: src/action_list.rs
line_hint: 90
author: 
created: 
title: "position"
tags: []
signature: "pub fn position (doc : & Document , list : & str , action : & str) -> Option < usize >"
---

Where the action is inside the list, if it is a child of it.

This is what `Move Up` and `Move Down` need to know: the position is the
argument of `Command::Reorder`, and asking twice would yield two different
answers the day the list changes between the two calls.

# Examples

```rust
use crawrad_actions::action_list::position;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert_eq!(position(&doc, "list1", "act1"), None);
```
