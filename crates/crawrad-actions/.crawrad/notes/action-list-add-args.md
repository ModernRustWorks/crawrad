---
id: action-list-add-args
kind: doc
target_file: src/action_list.rs
line_hint: 250
author: 
created: 
title: "add_args"
tags: []
signature: "pub fn add_args (doc : & Document , registry : & Registry , list : & str) -> Option < (& 'static str , Args) >"
---

The first button's action: a new Action, or a new item inside the collection.

# Examples

```rust
use crawrad_actions::action_list::add_args;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let mut doc = Document::new("form1", "Form");
doc.add_child("form1", Node::new("grid1", "Grid")).unwrap();
let (id, args) = add_args(&doc, &Registry::builtin(), "grid1").unwrap();
assert_eq!(id, "component.add");
assert_eq!(args.as_text("class"), Some("GridColumn"));
```
