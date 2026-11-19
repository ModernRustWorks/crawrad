---
id: inherited-form-newinheritedform-name-problem
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "NewInheritedForm::name_problem"
tags: []
signature: "pub fn name_problem (& self) -> Option < String >"
---

Why the typed name cannot be used, or `None` when it can.

The dialog shows the reason under the field and greys out Create.

# Examples

```
use crawrad_ir::Document;
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-doc-{}-{}", line!(), std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let base = Document::new("form_parent", "Form");
std::fs::write(dir.join("form_parent.crawform"), base.to_json().unwrap()).unwrap();
let mut m = NewInheritedForm::new(&dir, None);
m.name = "form_parent".into();
assert!(m.name_problem().is_some());
```
