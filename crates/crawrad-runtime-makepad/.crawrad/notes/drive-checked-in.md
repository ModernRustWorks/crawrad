---
id: drive-checked-in
kind: doc
target_file: src/drive.rs
line_hint: 1
author: 
created: 
title: "checked_in"
tags: []
signature: "pub fn checked_in (dlg : & WidgetRef , actions : & Actions , names : & [& str]) -> Option < bool >"
---

The first of these check boxes that changed: one setting shown on several pages.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Actions, Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dlg: &WidgetRef, actions: &Actions) {
    let _snap: Option<bool> = drive::checked_in(dlg, actions, &["opt_snap", "opt_cu_snap"]);
}
```
