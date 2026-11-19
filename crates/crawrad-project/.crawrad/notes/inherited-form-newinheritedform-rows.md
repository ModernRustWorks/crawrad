---
id: inherited-form-newinheritedform-rows
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "NewInheritedForm::rows"
tags: []
signature: "pub fn rows (& self) -> Vec < usize >"
---

The forms the list shows, as indexes into `forms`.

Those whose name has the filter, ignoring case.

# Examples

```
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-rows-{}", std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let mut m = NewInheritedForm::new(&dir, None);
m.filter = "nothing like this".into();
assert!(m.rows().is_empty());
```
