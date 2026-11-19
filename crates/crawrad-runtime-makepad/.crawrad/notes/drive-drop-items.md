---
id: drive-drop-items
kind: doc
target_file: src/drive.rs
line_hint: 105
author: 
created: 
title: "drop_items"
tags: []
signature: "pub fn drop_items (widget : & WidgetRef) -> Vec < String >"
---

The items `widget` was last filled with, when it is a drop-down.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::WidgetRef;

fn count(drop: &WidgetRef) -> usize {
    drive::drop_items(drop).len()
}
```
