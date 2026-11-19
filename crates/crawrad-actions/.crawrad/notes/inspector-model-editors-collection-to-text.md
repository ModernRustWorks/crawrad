---
id: inspector-model-editors-collection-to-text
kind: doc
target_file: src/inspector_model/editors/collection.rs
line_hint: 43
author: 
created: 
title: "to_text"
tags: []
signature: "pub fn to_text (items : & [Item]) -> String"
---

The collection written as lines, ready for the pad.

A leaf is its own text; a node is its text followed by its children, each
indented one level deeper.

# Examples

```rust
use crawrad_actions::inspector_model::editors::collection::to_text;
use crawrad_ir::Item;

let items = vec![Item::new("item1")];
assert_eq!(to_text(&items), "item1");
```
