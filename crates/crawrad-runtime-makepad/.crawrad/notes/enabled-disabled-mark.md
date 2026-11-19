---
id: enabled-disabled-mark
kind: doc
target_file: src/enabled.rs
line_hint: 27
author: 
created: 
title: "disabled_mark"
tags: []
signature: "pub fn disabled_mark (cx : & mut Cx , ui : & WidgetRef , path : & [LiveId] , checked : bool)"
---

Sets the disabled mark colour according to the checked state.

**A disabled CheckBox or RadioButton still says whether it is checked.**

Makepad 1.0.0's shader mixes the whole mark to `mark_color_disabled` by
the `disabled` animator alone, whatever `active` is
(`check_box.rs:189-197`, `radio_button.rs:192-200`): a disabled box that is
NOT checked is drawn with a tick, measured on a generated app (#431). So
while it is disabled, that colour follows `checked`.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{live_id, Cx, LiveId, WidgetRef};
use crawrad_runtime_makepad::enabled::disabled_mark;

fn update_checkbox(cx: &mut Cx, ui: &WidgetRef) {
    disabled_mark(cx, ui, &[live_id!(check1)], true);
}
```
