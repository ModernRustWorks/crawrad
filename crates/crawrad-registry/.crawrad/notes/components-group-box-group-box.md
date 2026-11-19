---
id: components-group-box-group-box
kind: doc
target_file: src/components/group_box.rs
line_hint: 4
author: 
created: 
title: "group_box"
tags: []
signature: "pub fn group_box () -> ComponentDesc"
---

`GroupBox`: the framed border with caption that **groups**.

# Examples

```rust
use crawrad_registry::group_box;

let desc = group_box();
assert_eq!(desc.class, "GroupBox");
```
