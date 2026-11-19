---
id: list-view-listviewcolumn
kind: doc
target_file: src/list_view.rs
line_hint: 113
author: 
created: 
title: "ListViewColumn"
tags: []
signature: "pub type ListViewColumn = (String , f32 , String)"
---

Column specification as `(caption, width, alignment)`.

# Examples

```rust
use crawrad_runtime_makepad::list_view::ListViewColumn;

let col: ListViewColumn = ("Name".into(), 100.0, "taLeftJustify".into());
assert_eq!(col.0, "Name");
```
