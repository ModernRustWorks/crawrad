---
id: members
kind: doc
target_file: src/lib.rs
line_hint: 159
author: 
created: 
title: "members"
tags: []
signature: "pub fn members (registry : & Registry , designer_source : Option < & str > , class : & str , prefix : & str ,) -> Vec < Member >"
---

Returns member fields matching a prefix for a given component class.

What to offer after `<field of class>.` with `prefix` typed: the fields
of `class` that start with `prefix` (ignoring case), in declaration order,
documented from the registry.

# Examples

```rust
use crawrad_insight::members;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let res = members(&reg, None, "Button", "te");
assert!(res.iter().any(|m| m.label == "text"));
```
