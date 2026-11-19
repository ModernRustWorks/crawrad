---
id: form-form-base-mut
kind: doc
target_file: src/form.rs
line_hint: 87
author: 
created: 
title: "Form::base_mut"
tags: []
signature: "fn base_mut (& mut self) -> & mut FormBase"
---

Returns a mutable reference to the underlying `FormBase`.

# Examples

```rust
use crawrad_runtime::{Form, FormBase};

struct MyForm(FormBase);
impl Form for MyForm {
    fn base(&self) -> &FormBase { &self.0 }
    fn base_mut(&mut self) -> &mut FormBase { &mut self.0 }
}

let mut form = MyForm(FormBase::default());
form.base_mut().width = 800.0;
assert_eq!(form.base().width, 800.0);
```
