---
id: clipboard-duplicate
kind: doc
target_file: src/clipboard.rs
line_hint: 108
author: 
created: 
title: "duplicate"
tags: []
signature: "pub fn duplicate (doc : & Document , sel : & [NodeId] , new_id : impl Fn (& str) -> String ,) -> Option < (Transaction , Vec < NodeId >) >"
---

Copies and pastes components in one transaction.

Duplicating is copying and pasting in one shot, without passing through the clipboard
— what was copied before remains there.

# Examples

```rust
use crawrad_designer::clipboard::duplicate;
use crawrad_ir::{Document, Node, NodeId};

let mut doc = Document::new("form", "Form");
let _ = doc.add_child("form", Node::new("btn", "Button"));
let res = duplicate(&doc, &[NodeId::from("btn")], |class| format!("{}_copy", class));
assert!(res.is_some());
```
