---
id: action-list-actions
kind: doc
target_file: src/action_list.rs
line_hint: 64
author: 
created: 
title: "actions"
tags: []
signature: "pub fn actions (doc : & Document , list : & str , category : & str) -> Vec < ActionRow >"
---

The list's actions falling into the chosen category.

`(All)` shows all; any other compares with the action's category, and an
action without a category counts as `(No Category)` — which is what makes the
pseudo-category mean something.

# Examples

```rust
use crawrad_actions::action_list::{actions, CAT_ALL};
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let rows = actions(&doc, "list1", CAT_ALL);
assert!(rows.is_empty());
```
