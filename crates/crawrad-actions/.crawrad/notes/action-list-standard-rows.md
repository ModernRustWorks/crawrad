---
id: action-list-standard-rows
kind: doc
target_file: src/action_list.rs
line_hint: 123
author: 
created: 
title: "standard_rows"
tags: []
signature: "pub fn standard_rows (closed : & dyn Fn (& str) -> bool , checked : & dyn Fn (& str) -> bool ,) -> Vec < StandardRow >"
---

The standard catalogue classes, flattened into list rows.

**The list belongs to `crawrad-registry`**, and no class is written here:
whoever adds a standard action to the catalogue sees it in this window
without a single line changing — the same rule as the Object Inspector and
the Toolbox.

# Examples

```rust
use crawrad_actions::action_list::standard_rows;

let rows = standard_rows(&|_| false, &|_| false);
assert!(!rows.is_empty());
```
