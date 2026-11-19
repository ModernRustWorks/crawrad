---
id: project-template-write-package
kind: doc
target_file: src/project_template.rs
line_hint: 242
author: 
created: 
title: "write_package"
tags: []
signature: "fn write_package (folder : & std :: path :: Path , name : & str , template_id : & str , options : & ProjectTemplateOptions ,) -> Result < () , String >"
---

Writes the project `name` from `template_id` into `folder`.

Writes the project `name` from `template_id` into the folder `root`: its
`Cargo.toml` and sources (the part shared by New Project and Add Project).
