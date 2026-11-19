---
id: choice
kind: doc
target_file: src/lib.rs
line_hint: 389
author: 
created: 
title: "Choice"
tags: []
signature: "pub struct Choice"
---

One value a parameter accepts.

Two strings and not one, for the reason [`Panel::id`] already gives: the
id is what crosses the protocol and must not move, the label is what a
person reads and is allowed to be retitled.

# Examples

```rust
use crawrad_actions::Choice;

let c = Choice { id: "explorer", label: "Explorer" };
assert_eq!(c.id, "explorer");
```
