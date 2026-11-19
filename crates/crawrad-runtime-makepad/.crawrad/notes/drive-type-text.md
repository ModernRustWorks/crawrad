---
id: drive-type-text
kind: doc
target_file: src/drive.rs
line_hint: 39
author: 
created: 
title: "type_text"
tags: []
signature: "pub fn type_text (cx : & mut Cx , root : & WidgetRef , path : & [LiveId] , value : & str ,) -> Result < () , Refused >"
---

Replaces the text of the field at `path` under `root` and reports it.

# Errors

`hidden-control` when the control was not drawn on screen.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dialog: &WidgetRef) {
    let _ = drive::type_text(cx, dialog, &[LiveId::from_str("opt_search_input")], "font");
}
```
