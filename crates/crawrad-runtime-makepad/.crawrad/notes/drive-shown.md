---
id: drive-shown
kind: doc
target_file: src/drive.rs
line_hint: 28
author: 
created: 
title: "shown"
tags: []
signature: "pub fn shown (cx : & Cx , widget : & WidgetRef) -> Result < () , Refused >"
---

`Ok` when `widget` was drawn on screen, as a user would see it.

A widget can say `visible` while its page is hidden; only a drawn one has
a rect with a size.

# Errors

`hidden-control` when it was not drawn.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Cx, WidgetRef};

fn usable(cx: &Cx, widget: &WidgetRef) -> bool {
    drive::shown(cx, widget).is_ok()
}
```
