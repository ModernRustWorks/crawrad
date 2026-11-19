---
id: action-list-add-all-fields-args
kind: doc
target_file: src/action_list.rs
line_hint: 260
author: 
created: 
title: "add_all_fields_args"
tags: []
signature: "pub fn add_all_fields_args (node : & str , fields : & [String] , kinds : & [String]) -> Args"
---

Arguments of `collection.add_all_fields` for a grid and its table's fields.

# Examples

```rust
use crawrad_actions::action_list::add_all_fields_args;

let args = add_all_fields_args("grid1", &["id".into()], &["INTEGER".into()]);
assert_eq!(args.as_text("node"), Some("grid1"));
assert_eq!(args.as_texts("fields"), vec!["id"]);
```
