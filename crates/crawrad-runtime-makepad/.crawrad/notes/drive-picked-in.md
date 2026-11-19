---
id: drive-picked-in
kind: doc
target_file: src/drive.rs
line_hint: 1
author: 
created: 
title: "picked_in"
tags: []
signature: "pub fn picked_in (dlg : & WidgetRef , actions : & Actions , names : & [& str]) -> Option < usize >"
---

Finds the first drop-down whose item was picked.

The first of these drop-downs whose item was picked: one setting on several pages.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Actions, Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dlg: &WidgetRef, actions: &Actions) {
    let _size: Option<usize> = drive::picked_in(dlg, actions, &["opt_cu_font_size"]);
}
```
