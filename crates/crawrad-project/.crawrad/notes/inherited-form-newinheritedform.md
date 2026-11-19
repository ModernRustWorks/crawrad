---
id: inherited-form-newinheritedform
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "NewInheritedForm"
tags: []
signature: "pub struct NewInheritedForm"
---

What the New Inherited Form dialog shows and creates (#851).

The list of forms, the base picked, and the name typed: the dialog paints it
and turns clicks into changes of these fields.

# Examples

```
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-model-{}", std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let m = NewInheritedForm::new(&dir, None);
assert!(m.base().is_none());
assert!(!m.can_create());
```
