---
id: form-formbase
kind: doc
target_file: src/form.rs
line_hint: 10
author: 
created: 
title: "FormBase"
tags: []
signature: "pub struct FormBase"
---

Foundational state for any window or form.

Direct architectural equivalent to a classic RAD IDE `TForm`.
Wraps window caption, dimensions, key handling preferences,
and dialog modal results.

# Examples

```rust
use crawrad_runtime::FormBase;

let form = FormBase::default();
assert_eq!(form.title, "Form");
```
