---
id: targets-makepad-form-hint-formhint
kind: doc
target_file: src/targets/makepad/form_hint.rs
line_hint: 21
author: 
created: 
title: "FormHint"
tags: []
signature: "pub struct FormHint"
---

Code and state generated for the form's hint support in `App`.

What the form's hint needs in the `App`: the flag that tells the tooltip
is the form's, and the block after the widgets. `None` when the form has
no hint to show.

# Examples

```rust
use crawrad_codegen::targets::makepad::form_hint::FormHint;

let hint = FormHint {
    field: String::new(),
    after_widgets: String::new(),
};
assert!(hint.field.is_empty());
```
