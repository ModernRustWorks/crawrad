---
id: drive-drop-text
kind: doc
target_file: src/drive.rs
line_hint: 111
author: 
created: 
title: "drop_text"
tags: []
signature: "pub fn drop_text (widget : & WidgetRef) -> Option < String >"
---

The selected item's text of a drop-down; `None` for any other widget.

# Examples

```no_run
use crawrad_runtime_makepad::drive;
use crawrad_runtime_makepad::makepad_widgets::WidgetRef;

fn text(widget: &WidgetRef) -> String {
    drive::drop_text(widget).unwrap_or_else(|| widget.text())
}
```
