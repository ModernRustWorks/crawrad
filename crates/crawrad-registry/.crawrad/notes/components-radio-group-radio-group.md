---
id: components-radio-group-radio-group
kind: doc
target_file: src/components/radio_group.rs
line_hint: 4
author: 
created: 
title: "radio_group"
tags: []
signature: "pub fn radio_group () -> ComponentDesc"
---

A framed group of exclusive radio button options.

`RadioGroup` (#500, wave 1): a framed group of exclusive options, one
control — its `Items` are the options, `ItemIndex` the chosen one (-1:
none), laid out in `Columns` columns, as in the classic RAD IDE. Unlike
loose `RadioButton`s, the group is one field the form reads and sets.

# Examples

```rust
use crawrad_registry::radio_group;

let desc = radio_group();
assert_eq!(desc.class, "RadioGroup");
```
