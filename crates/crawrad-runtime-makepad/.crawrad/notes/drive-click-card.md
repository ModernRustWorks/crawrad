---
id: drive-click-card
kind: doc
target_file: src/drive.rs
line_hint: 80
author: 
created: 
title: "click_card"
tags: []
signature: "pub fn click_card (cx : & mut Cx , root : & WidgetRef , path : & [LiveId]) -> Result < () , Refused >"
---

Clicks the card at `path` under `root`, as a tap on its surface does.

# Errors

`hidden-control` when the card was not drawn on screen.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::{Cx, LiveId, WidgetRef};

fn act(cx: &mut Cx, dialog: &WidgetRef) {
    let _ = drive::click_card(cx, dialog, &[LiveId::from_str("np_card_0")]);
}
```
