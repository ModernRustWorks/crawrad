---
id: inherited-form-newinheritedform-create
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "NewInheritedForm::create"
tags: []
signature: "pub fn create (& self) -> Result < PathBuf , String >"
---

Writes the new form: its root only, inheriting the picked base.

Everything else comes from the base when the designer opens it, so the file
holds only what the new form changes later.

# Errors

The reason the name cannot be used, or why the file could not be written.

# Examples

```
use crawrad_ir::Document;
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-doc-{}-{}", line!(), std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let base = Document::new("form_parent", "Form");
std::fs::write(dir.join("form_parent.crawform"), base.to_json().unwrap()).unwrap();
let path = NewInheritedForm::new(&dir, None).create().unwrap();
assert_eq!(Document::load(&path).unwrap().inherits(), Some("form_parent"));
```
