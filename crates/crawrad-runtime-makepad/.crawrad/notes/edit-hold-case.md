---
id: edit-hold-case
kind: doc
target_file: src/edit.rs
line_hint: 47
author: 
created: 
title: "hold_case"
tags: []
signature: "pub fn hold_case (cx : & mut Cx , input : & TextInputRef , text : & str , char_case : & str) -> Option < String >"
---

Enforces character casing rules on a TextInput.

Holds the input to its `CharCase` (#667, the rule is
`crawrad_ir::char_case`): the text turned, or `None` when it already was. The
caret stays where it was when the turn keeps the length, and goes to the
end when it does not (`ß` is `SS`).

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, TextInputRef};
use crawrad_runtime_makepad::edit::hold_case;

fn example(cx: &mut Cx, input: &TextInputRef) {
    let _ = hold_case(cx, input, "hello", "ecUpperCase");
}
```
