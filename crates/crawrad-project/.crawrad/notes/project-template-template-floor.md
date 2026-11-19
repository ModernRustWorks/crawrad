---
id: project-template-template-floor
kind: doc
target_file: src/project_template.rs
line_hint: 206
author: 
created: 
title: "template_floor"
tags: []
signature: "fn template_floor (template_id : & str , options : & ProjectTemplateOptions) -> (u32 , u32)"
---

Returns the toolchain minimum version floor for a template.

What `write_package` will put in the project, as far as the toolchain cares:
its target and whether it has a form. Mirrors the arms of `write_package` —
the three without a form, and the three that are always Makepad.
