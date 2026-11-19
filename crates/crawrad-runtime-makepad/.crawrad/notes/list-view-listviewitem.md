---
id: list-view-listviewitem
kind: doc
target_file: src/list_view.rs
line_hint: 116
author: 
created: 
title: "ListViewItem"
tags: []
signature: "pub struct ListViewItem"
---

One row of a `ListView` (#500).

# Examples

```rust
use crawrad_runtime_makepad::list_view::ListViewItem;

let item = ListViewItem {
    text: "Row 1".into(),
    sub_items: vec!["Detail".into()],
};
assert_eq!(item.text, "Row 1");
```
