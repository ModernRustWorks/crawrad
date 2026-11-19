---
id: items-editor-itempath
kind: doc
target_file: src/items_editor.rs
line_hint: 34
author: 
created: 
title: "ItemPath"
tags: []
signature: "pub type ItemPath = Vec < usize >"
---

Indices from the root down to an item.

The path to an item: indices from top inwards. `[0, 2]` is the third child
of the first item.

# Examples

```rust
use crawrad_actions::items_editor::ItemPath;

let path: ItemPath = vec![0, 2];
assert_eq!(path.len(), 2);
```
