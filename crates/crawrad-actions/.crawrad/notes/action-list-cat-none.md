---
id: action-list-cat-none
kind: doc
target_file: src/action_list.rs
line_hint: 38
author: 
created: 
title: "CAT_NONE"
tags: []
signature: "pub const CAT_NONE : & str = \"(No Category)\""
---

The pseudo-category for actions that did not declare any.

# Examples

```rust
use crawrad_actions::action_list::CAT_NONE;

assert_eq!(CAT_NONE, "(No Category)");
```
