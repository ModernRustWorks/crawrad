---
id: inherited-form-newinheritedform-new
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "NewInheritedForm::new"
tags: []
signature: "pub fn new (dir : & Path , base : Option < & str >) -> Self"
---

Opens the dialog's model on a folder, with `base` picked when it is there.

The name starts as the first free `form_N`, as `New Form` names its forms.

# Examples

```
use crawrad_ir::Document;
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-doc-{}-{}", line!(), std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let base = Document::new("form_parent", "Form");
std::fs::write(dir.join("form_parent.crawform"), base.to_json().unwrap()).unwrap();
let m = NewInheritedForm::new(&dir, Some("form_parent"));
assert_eq!(m.base().map(|b| b.name.as_str()), Some("form_parent"));
assert_eq!(m.name, "form_1");
```
