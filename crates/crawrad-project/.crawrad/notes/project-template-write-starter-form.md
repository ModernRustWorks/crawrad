---
id: project-template-write-starter-form
kind: doc
target_file: src/project_template.rs
line_hint: 419
author: 
created: 
title: "write_starter_form"
tags: []
signature: "fn write_starter_form (member : & std :: path :: Path , framework : crawrad_codegen :: AlvoGerador ,) -> Result < () , String >"
---

A GUI application is born WITH its form (#581, #584, #585).

Without one, the IDE kept whatever form was open before as the document,
`Generate` wrote that stale form into the new project, `Run` executed the
template's demo, and nothing of the design existed on disk. The form is the
blank template — the same document `New Project…` saves — and the code is
generated with it, so `main.rs` already shows the form.
