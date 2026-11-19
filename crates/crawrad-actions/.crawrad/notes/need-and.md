---
id: need-and
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "Need::and"
tags: []
signature: "pub fn and (self , check : fn (& Context) -> bool) -> Gate"
---

The gate of an action that needs this and passes `check` too.

# Examples

```rust
use crawrad_actions::Need;

let gate = Need::Selection.and(|c| c.doc.is_some_and(|d| d.selection().len() >= 2));
assert_eq!(gate.need, Need::Selection);
```
