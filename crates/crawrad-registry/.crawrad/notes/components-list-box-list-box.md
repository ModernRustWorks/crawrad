---
id: components-list-box-list-box
kind: doc
target_file: src/components/list_box.rs
line_hint: 4
author: 
created: 
title: "list_box"
tags: []
signature: "pub fn list_box () -> ComponentDesc"
---

A list box showing a scrollable list of items.

`ListBox`: the open list. The same collection as `ComboBox`, with the same
`ItemIndex` — the third time `crawrad_ir::Item` pays its own cost.

# Examples

```rust
use crawrad_registry::list_box;

let desc = list_box();
assert_eq!(desc.class, "ListBox");
```
