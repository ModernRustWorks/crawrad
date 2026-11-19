---
id: components-chips-chips
kind: doc
target_file: src/components/chips.rs
line_hint: 4
author: 
created: 
title: "chips"
tags: []
signature: "pub fn chips () -> ComponentDesc"
---

A group of small rounded chips laid out in rows.

`Chips`: a group of small rounded chips laid out left to right,
wrapping to the next line when the row is full (#500).

# Examples

```rust
use crawrad_registry::chips;

let desc = chips();
assert_eq!(desc.class, "Chips");
```
