---
id: inherited-form-forms-in
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "forms_in"
tags: []
signature: "pub fn forms_in (dir : & Path) -> Vec < FormEntry >"
---

The forms in a folder, each derived one right under its base (#851).

A form whose base is not in the folder is listed at the top level.

# Examples

```
use crawrad_ir::Document;
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-doc-{}-{}", line!(), std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let base = Document::new("form_parent", "Form");
std::fs::write(dir.join("form_parent.crawform"), base.to_json().unwrap()).unwrap();
use crawrad_project::inherited_form::forms_in;

let child = Document::new_inherited("form_child", "Form", "form_parent");
std::fs::write(dir.join("form_child.crawform"), child.to_json().unwrap()).unwrap();
let forms = forms_in(&dir);
assert_eq!(forms[1].name, "form_child");
assert_eq!(forms[1].depth, 1);
```
