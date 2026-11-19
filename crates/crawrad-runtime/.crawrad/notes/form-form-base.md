---
id: form-form-base
kind: doc
target_file: src/form.rs
line_hint: 84
author: 
created: 
title: "Form::base"
tags: []
signature: "fn base (& self) -> & FormBase"
---

Returns a reference to the underlying `FormBase`.

# Examples

```rust
use crawrad_runtime::{Form, FormBase};

struct MyForm(FormBase);
impl Form for MyForm {
    fn base(&self) -> &FormBase { &self.0 }
    fn base_mut(&mut self) -> &mut FormBase { &mut self.0 }
}

let form = MyForm(FormBase::default());
assert_eq!(form.base().width, 640.0);
```
