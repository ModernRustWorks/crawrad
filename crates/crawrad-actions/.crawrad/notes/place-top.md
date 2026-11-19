---
id: place-top
kind: doc
target_file: src/lib.rs
line_hint: 1177
author: 
created: 
title: "Place::TOP"
tags: []
signature: "pub const TOP : Self = Self :: section (0)"
---

The first section of the menu itself — every action that says nothing.

# Examples

```rust
use crawrad_actions::Place;

assert_eq!(Place::TOP.section, 0);
```
