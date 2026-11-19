---
id: list-view-aslistviewitem
kind: doc
target_file: src/list_view.rs
line_hint: 135
author: 
created: 
title: "AsListViewItem"
tags: []
signature: "pub trait AsListViewItem"
---

Trait implemented by any row item that can be shown in a `ListView`.

# Examples

```rust
use crawrad_runtime_makepad::list_view::{AsListViewItem, ListViewItem};

let item = ListViewItem { text: "Main".into(), sub_items: Vec::new() };
assert_eq!(item.text(), "Main");
```
