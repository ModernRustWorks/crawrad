---
id: ide-app-dialogs-exit-ideapp-has-anything-to-save
kind: doc
target_file: src/ide_app/dialogs/exit.rs
line_hint: 11
author: 
created: 
title: "IdeApp::has_anything_to_save"
tags: []
signature: "pub (in super :: super) fn has_anything_to_save (& self) -> bool"
---

Checks whether there is work to save in the project or editors.

Is there work to save?

The document revision against the saved revision, which is the same yardstick
as the title's `saved`/`modified` stamp and the same one that
[`IdeApp::abrir_formulario`] uses to refuse switching forms. A
second idea of *dirty* here would mean the title saying one thing and the
dialog another.
