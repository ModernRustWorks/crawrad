---
id: inherited-form-newinheritedform-can-create
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "NewInheritedForm::can_create"
tags: []
signature: "pub fn can_create (& self) -> bool"
---

There is a base picked and the name is good: Create is enabled.

# Examples

```
use crawrad_ir::Document;
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-doc-{}-{}", line!(), std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let base = Document::new("form_parent", "Form");
std::fs::write(dir.join("form_parent.crawform"), base.to_json().unwrap()).unwrap();
assert!(NewInheritedForm::new(&dir, None).can_create());
```
