---
id: drive-press
kind: doc
target_file: src/drive.rs
line_hint: 12
author: 
created: 
title: "press"
tags: []
signature: "pub fn press (cx : & mut Cx , root : & WidgetRef , path : & [LiveId]) -> Result < () , Refused >"
---

Presses the button at `path` under `root`.

# Errors

`hidden-control` when the control was not drawn on screen.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dialog: &WidgetRef) {
    let _ = drive::press(cx, dialog, &[LiveId::from_str("opt_close")]);
}
```
