---
id: label-apply
kind: doc
target_file: src/label.rs
line_hint: 38
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (cx : & mut Cx , view : & WidgetRef , label : & LabelRef , look : & LabelLook)"
---

Applies alignment, wrapping, and sizing to a label view and its inner label.

Applies the look to a generated label: `view` is the outer `View`, `label`
the makepad `Label` inside it.

# Examples

```rust,no_run
// Makepad Cx, WidgetRef, and LabelRef require the Makepad application runtime.
use makepad_widgets::{Cx, LabelRef, WidgetRef};
use crawrad_runtime_makepad::label::{apply, LabelLook};

fn update_label(cx: &mut Cx, view: &WidgetRef, label: &LabelRef) {
    let look = LabelLook {
        alignment: "taLeftJustify",
        layout: "tlTop",
        word_wrap: false,
        auto_size: true,
        size: (100.0, 20.0),
    };
    apply(cx, view, label, &look);
}
```
