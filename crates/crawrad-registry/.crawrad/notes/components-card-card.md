---
id: components-card-card
kind: doc
target_file: src/components/card.rs
line_hint: 5
author: 
created: 
title: "card"
tags: []
signature: "pub fn card () -> ComponentDesc"
---

A container with rounded corners, elevation shadow, and optional header.

`Card` (#500, wave 2): a modern container with rounded corners, elevation
shadow, optional header with title and subtitle; its children keep the designer's positions.

# Examples

```rust
use crawrad_registry::card;

let desc = card();
assert_eq!(desc.class, "Card");
```
