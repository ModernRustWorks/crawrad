---
id: inherited-form-newinheritedform-files
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "NewInheritedForm::files"
tags: []
signature: "pub fn files (& self) -> (String , String)"
---

The files the new form is born as, and the struct its code gets.

# Examples

```
use crawrad_ir::Document;
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-doc-{}-{}", line!(), std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let base = Document::new("form_parent", "Form");
std::fs::write(dir.join("form_parent.crawform"), base.to_json().unwrap()).unwrap();
let (_files, ty) = NewInheritedForm::new(&dir, None).files();
assert!(ty.contains("base: FormParent"));
```
