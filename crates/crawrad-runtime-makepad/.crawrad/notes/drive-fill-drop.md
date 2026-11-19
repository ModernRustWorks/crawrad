---
id: drive-fill-drop
kind: doc
target_file: src/drive.rs
line_hint: 1
author: 
created: 
title: "fill_drop"
tags: []
signature: "pub fn fill_drop (cx : & mut Cx , dlg : & WidgetRef , drop : & [LiveId] , labels : Vec < String > , at : Option < usize >)"
---

A drop-down given its items and, when there is one, the item picked.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Actions, Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dlg: &WidgetRef, actions: &Actions) {
    drive::fill_drop(cx, dlg, &[LiveId::from_str("opt_cu_font_size")], vec!["10 pt".into()], Some(0));
}
```
