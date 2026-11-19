---
id: action-list-rows
kind: doc
target_file: src/action_list.rs
line_hint: 241
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (doc : & Document , list : & str , category : & str) -> (Vec < String > , Vec < ActionRow >)"
---

Rows of the dialog: categories and actions, or no categories and the items.

# Examples

```rust
use crawrad_actions::action_list::rows;
use crawrad_ir::{Document, Node};

let mut doc = Document::new("form1", "Form");
doc.add_child("form1", Node::new("grid1", "Grid")).unwrap();
doc.add_child("grid1", Node::new("col1", "GridColumn")).unwrap();
let (cats, items) = rows(&doc, "grid1", "");
assert!(cats.is_empty());
assert_eq!(items[0].label, "0  col1");
```
