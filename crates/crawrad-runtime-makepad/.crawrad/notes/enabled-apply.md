---
id: enabled-apply
kind: doc
target_file: src/enabled.rs
line_hint: 38
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (cx : & mut Cx , ui : & WidgetRef , path : & [LiveId] , enabled : bool)"
---

Puts widget `path` under `ui` in the `enabled` state.

It only acts when the state changes: the sync runs on every pass, and a
redraw per pass would keep the app drawing forever.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{live_id, Cx, LiveId, WidgetRef};
use crawrad_runtime_makepad::enabled::apply;

fn set_enabled_state(cx: &mut Cx, ui: &WidgetRef, enabled: bool) {
    apply(cx, ui, &[live_id!(btn_save)], enabled);
}
```
