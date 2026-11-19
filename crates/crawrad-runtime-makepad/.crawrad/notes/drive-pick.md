---
id: drive-pick
kind: doc
target_file: src/drive.rs
line_hint: 89
author: 
created: 
title: "pick"
tags: []
signature: "pub fn pick (cx : & mut Cx , root : & WidgetRef , path : & [LiveId] , label : & str) -> Result < () , Refused >"
---

Picks the item whose text is `label` in the drop-down at `path`.

Emits the `DropDownAction::Select` a real pick sends.

# Errors

`hidden-control` when the drop-down was not drawn on screen, and
`unknown-item` when no item has that text — nothing changes then.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dialog: &WidgetRef) {
    let path = [LiveId::from_str("opt_cu_font_size")];
    let _ = drive::pick(cx, dialog, &path, "12 pt");
}
```
