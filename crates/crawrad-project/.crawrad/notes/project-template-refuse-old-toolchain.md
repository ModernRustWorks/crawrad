---
id: project-template-refuse-old-toolchain
kind: doc
target_file: src/project_template.rs
line_hint: 219
author: 
created: 
title: "refuse_old_toolchain"
tags: []
signature: "fn refuse_old_toolchain (at : & std :: path :: Path , template_id : & str , options : & ProjectTemplateOptions ,) -> Result < () , String >"
---

Checks whether the installed toolchain satisfies the template requirements.

**The toolchain is asked BEFORE anything is written**: an old one would get
the folder and then fail on the first build with a message about `edition2024`
or a dependency's `rust-version`, far from the gesture that caused it. When
`cargo --version` cannot be read, nothing is refused — the build speaks then.
