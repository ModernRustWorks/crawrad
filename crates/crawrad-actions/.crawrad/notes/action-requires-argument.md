---
id: action-requires-argument
kind: doc
target_file: src/lib.rs
line_hint: 530
author: 
created: 
title: "Action::requires_argument"
tags: []
signature: "pub fn requires_argument (& self) -> bool"
---

Returns whether the action requires an argument to execute.

Does the action **require** an argument to do anything?

It is what separates whoever can become a menu item from whoever cannot: a menu
has nowhere to type an argument, so drawing an action that requires one
produces a lit and inert item. Whoever requires an argument appears in the
interface through a doorway that knows how to collect it — a dialog, the Explorer,
the MCP.

# Examples

```rust
use crawrad_actions::by_id;

let action = by_id("file.save").unwrap();
assert!(!action.requires_argument());
```
