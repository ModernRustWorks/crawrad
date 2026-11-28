---
id: project-template-refuse-a-platform-the-framework-cannot-reach
kind: doc
target_file: src/project_template.rs
line_hint: 231
author: 
created: 
title: "refuse_a_platform_the_framework_cannot_reach"
tags: []
signature: "fn refuse_a_platform_the_framework_cannot_reach (template_id : & str, options : & ProjectTemplateOptions) -> Result < (), String >"
---

Refuses a template the chosen framework cannot build for, naming which.

Three templates used to hardcode `AlvoGerador::Makepad` and ignore
`options.framework` entirely, so choosing **egui + WASM** quietly produced a
Makepad project: the dialog offered a combination it did not honour, and nothing
said so. Handing the person another framework's project is worse than refusing -
they find out when the code does not look like what they asked for.

The decision reads the framework card's own `web` and `mobile`, not a name
written here, so a framework added later is covered without touching this.

# Errors

A phrase naming the framework and the platform it cannot reach, for the dialog
to show.
