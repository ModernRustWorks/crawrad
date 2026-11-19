---
id: drive-toggle
kind: doc
target_file: src/drive.rs
line_hint: 33
author: 
created: 
title: "toggle"
tags: []
signature: "pub fn toggle (cx : & mut Cx , root : & WidgetRef , path : & [LiveId]) -> Result < () , Refused >"
---

Flips the check box at `path` under `root`, as a click on it does.

# Errors

`hidden-control` when the control was not drawn on screen.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dialog: &WidgetRef) {
    let _ = drive::toggle(cx, dialog, &[LiveId::from_str("opt_snap")]);
}
```
