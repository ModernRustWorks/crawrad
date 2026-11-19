---
id: edit-hold-mask
kind: doc
target_file: src/edit.rs
line_hint: 63
author: 
created: 
title: "hold_mask"
tags: []
signature: "pub fn hold_mask (cx : & mut Cx , input : & TextInputRef , before : & str , text : & str , edit_mask : & str ,) -> Option < String >"
---

Applies edit mask formatting to text and updates the caret.

Holds the input to its `EditMask` (#71, #21, the rule is `crawrad_runtime::mask`):
the text shaped by the mask and the caret on the next editable
position, or `None` when there is no mask.

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::edit::hold_mask;

fn example(cx: &mut Cx, input: &TextInputRef) {
    let _ = hold_mask(cx, input, "", "12", "00/00");
}
```
