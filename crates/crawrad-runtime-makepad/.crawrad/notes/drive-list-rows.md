---
id: drive-list-rows
kind: doc
target_file: src/drive.rs
line_hint: 77
author: 
created: 
title: "list_rows"
tags: []
signature: "pub fn list_rows (cx : & Cx , root : & WidgetRef , name : & str) -> Vec < (String , Rect) >"
---

The rows the list box named `name` under `root` drew, with their rects.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Cx, WidgetRef};

fn rows(cx: &Cx, dialog: &WidgetRef) -> usize {
    drive::list_rows(cx, dialog, "np_cat_list").len()
}
```
