---
id: components-action-list-action-list
kind: doc
target_file: src/components/action_list.rs
line_hint: 6
author: 
created: 
title: "action_list"
tags: []
signature: "pub fn action_list () -> ComponentDesc"
---

Stores the form's actions and the icon list they use.

`ActionList` stores the form's actions and the icon list they
use. The actions are its **children** — in the classic RAD IDE they sit in a separate editor, and
here they appear in the structure tree, which amounts to the same with fewer windows.

# Examples

```rust
use crawrad_registry::action_list;

let desc = action_list();
assert_eq!(desc.class, "ActionList");
```
