---
id: components-action-list-action
kind: doc
target_file: src/components/action_list.rs
line_hint: 33
author: 
created: 
title: "action"
tags: []
signature: "pub fn action () -> ComponentDesc"
---

Defines an individual action that controls or buttons can fire.

`Action` is what the button fires. Caption, hint, shortcut, and icon live here —
and from here connected components inherit all of that.

# Examples

```rust
use crawrad_registry::action;

let desc = action();
assert_eq!(desc.class, "Action");
```
