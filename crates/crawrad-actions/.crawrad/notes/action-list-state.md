---
id: action-list-state
kind: doc
target_file: src/action_list.rs
line_hint: 211
author: 
created: 
title: "state"
tags: []
signature: "pub fn state (doc : & Document , registry : & Registry , list : & str , selection : & [String] , columns : Option < & [String] > ,) -> DialogState"
---

The dialog's state; `columns` are the grid's design-time fields, if any.

# Examples

```rust
use crawrad_actions::action_list::state;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let mut doc = Document::new("form1", "Form");
doc.add_child("form1", Node::new("grid1", "Grid")).unwrap();
let cols = vec!["id".to_string()];
let st = state(&doc, &Registry::builtin(), "grid1", &[], Some(&cols));
assert!(st.can_add_fields);
assert!(!st.can_delete);
```
