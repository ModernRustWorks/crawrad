---
id: drive-set-checked
kind: doc
target_file: src/drive.rs
line_hint: 22
author: 
created: 
title: "set_checked"
tags: []
signature: "pub fn set_checked (cx : & mut Cx , root : & WidgetRef , path : & [LiveId] , value : bool ,) -> Result < () , Refused >"
---

Sets the check box at `path` under `root` and reports the change.

# Errors

`hidden-control` when the control was not drawn on screen.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dialog: &WidgetRef) {
    let _ = drive::set_checked(cx, dialog, &[LiveId::from_str("opt_snap")], true);
}
```
