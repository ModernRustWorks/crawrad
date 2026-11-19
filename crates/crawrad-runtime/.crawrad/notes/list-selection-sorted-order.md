---
id: list-selection-sorted-order
kind: doc
target_file: src/list_selection.rs
line_hint: 58
author: 
created: 
title: "sorted_order"
tags: []
signature: "pub fn sorted_order < S : AsRef < str > > (items : & [S]) -> Vec < usize >"
---

Returns the index order of items when sorted case-insensitively.

The rows of `items` in `Sorted` order: position `n` holds the index of the
row that goes there. The designer draws by it without touching the items.

# Examples

```rust
use crawrad_runtime::list_selection::sorted_order;

let items = ["Banana", "apple"];
let order = sorted_order(&items);
assert_eq!(order, vec![1, 0]);
```
