---
id: menu-nothing-here
kind: doc
target_file: src/menu.rs
line_hint: 96
author: 
created: 
title: "NOTHING_HERE"
tags: []
signature: "pub const NOTHING_HERE : & str = \"no command here yet\""
---

What a menu says when the group has no drawable command.

A popup with zero rows is a box with no height — invisible, and
indistinguishable from a menu that failed to open. It should never appear
today (`crawrad-actions` has a test that fails when a menu group empties out),
and it exists so that the day it does, the screen says so.

# Examples

```rust
use crawrad_actions::menu::NOTHING_HERE;

assert_eq!(NOTHING_HERE, "no command here yet");
```
