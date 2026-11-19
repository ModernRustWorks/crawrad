---
id: form-form
kind: doc
target_file: src/form.rs
line_hint: 82
author: 
created: 
title: "Form"
tags: []
signature: "pub trait Form"
---

Contract for forms, mirroring the classic RAD `TForm`.

Trait implemented by all generated and custom forms, mirroring the classic RAD `TForm` contract.

# Examples

```rust
use crawrad_runtime::{Form, FormBase};

struct MyForm(FormBase);
impl Form for MyForm {
    fn base(&self) -> &FormBase { &self.0 }
    fn base_mut(&mut self) -> &mut FormBase { &mut self.0 }
}

let form = MyForm(FormBase::new("Custom", 320.0, 240.0));
assert_eq!(form.title(), "Custom");
```
