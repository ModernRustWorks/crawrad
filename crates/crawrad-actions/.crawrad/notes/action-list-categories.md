---
id: action-list-categories
kind: doc
target_file: src/action_list.rs
line_hint: 41
author: 
created: 
title: "categories"
tags: []
signature: "pub fn categories (doc : & Document , list : & str) -> Vec < String >"
---

Returns the category list for the given action list.

The left column categories: the two fixed ones, and then those declared by
the list's actions.

**Without duplicates and in the order they appear**, which is document order
— not alphabetical. Sorting here would shuffle the column when the user
changes an action's category, and what they expect is that it stays where it
was.

# Examples

```rust
use crawrad_actions::action_list::{categories, CAT_ALL, CAT_NONE};
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let cats = categories(&doc, "list1");
assert_eq!(cats, vec![CAT_ALL.to_string(), CAT_NONE.to_string()]);
```
