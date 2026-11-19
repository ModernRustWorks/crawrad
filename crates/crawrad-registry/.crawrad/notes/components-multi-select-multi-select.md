---
id: components-multi-select-multi-select
kind: doc
target_file: src/components/multi_select.rs
line_hint: 4
author: 
created: 
title: "multi_select"
tags: []
signature: "pub fn multi_select () -> ComponentDesc"
---

A drop-down selection list where multiple items can be checked.

`MultiSelect`: a drop-down field whose list has a check box per item;
any number of items can be checked (#500).

# Examples

```rust
use crawrad_registry::multi_select;

let desc = multi_select();
assert_eq!(desc.class, "MultiSelect");
```
