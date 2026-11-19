---
id: inherited-form-formentry
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "FormEntry"
tags: []
signature: "pub struct FormEntry"
---

One form of the project, as the New Inherited Form dialog lists it (#851).

`depth` is how far under its base the row is drawn, so a chain A > B > C
reads at a glance.

# Examples

```
use crawrad_project::inherited_form::FormEntry;

let f = FormEntry {
    name: "form_child".into(),
    path: "src/form_child.crawform".into(),
    base: Some("form_parent".into()),
    depth: 1,
};
assert_eq!(f.base.as_deref(), Some("form_parent"));
```
