---
id: inherited-form-newinheritedform-summary
kind: doc
target_file: src/inherited_form.rs
line_hint: 1
author:
created:
title: "NewInheritedForm::summary"
tags: []
signature: "pub fn summary (& self) -> Option < (String , String) >"
---

What the picked base brings: a count line and the components' names.

It counts the base as the designer opens it, with what the base itself
inherits.

# Examples

```
use crawrad_ir::Document;
use crawrad_project::inherited_form::NewInheritedForm;

let dir = std::env::temp_dir().join(format!("crawrad-inherited-doc-{}-{}", line!(), std::process::id()));
std::fs::create_dir_all(&dir).unwrap();
let base = Document::new("form_parent", "Form");
std::fs::write(dir.join("form_parent.crawform"), base.to_json().unwrap()).unwrap();
let (line, _names) = NewInheritedForm::new(&dir, None).summary().unwrap();
assert!(line.starts_with("form_parent"));
```
