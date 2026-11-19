---
id: progress-apply
kind: doc
target_file: src/progress.rs
line_hint: 20
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (cx : & mut Cx , ui : & WidgetRef , path : & [LiveId] , fraction : f64 , vertical : bool , width : f64 , height : f64 ,)"
---

Sizes the inner progress bar widget according to the current fraction.

Sizes the `bar` inside ProgressBar `path` to `fraction` of the control's
`width` × `height` (its design size): the width, or the height when
`vertical`.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{live_id, Cx, LiveId, WidgetRef};
use crawrad_runtime_makepad::progress::apply;

fn update_progress(cx: &mut Cx, ui: &WidgetRef) {
    apply(cx, ui, &[live_id!(bar1)], 0.5, false, 200.0, 20.0);
}
```
