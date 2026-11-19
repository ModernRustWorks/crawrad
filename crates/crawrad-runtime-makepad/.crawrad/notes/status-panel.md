---
id: status-panel
kind: doc
target_file: src/status.rs
line_hint: 7
author: 
created: 
title: "panel"
tags: []
signature: "pub fn panel (cx : & mut Cx , cell : & WidgetRef , label : & LabelRef , text : & str , width : i64 , alignment : & str ,)"
---

Configures a status bar panel's text, width, and alignment.

Gives a panel its `text`, its `width` in points and its `alignment`
(`taLeftJustify`, `taCenter`, `taRightJustify`). `cell` is the panel's
box (a View that aligns) and `label` the text inside it, which fits its
text: a fixed-width Label pushed right-aligned text past its own edge on
makepad 1.0.0. The text is set only when it changed.

# Examples

```rust,no_run
// Makepad Cx, WidgetRef, and LabelRef require the Makepad application runtime.
use makepad_widgets::{Cx, LabelRef, WidgetRef};
use crawrad_runtime_makepad::status::panel;

fn update_panel(cx: &mut Cx, cell: &WidgetRef, label: &LabelRef) {
    panel(cx, cell, label, "Ready", 100, "taLeftJustify");
}
```
