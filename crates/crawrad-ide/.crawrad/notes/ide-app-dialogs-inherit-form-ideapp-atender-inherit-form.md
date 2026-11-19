---
id: ide-app-dialogs-inherit-form-ideapp-atender-inherit-form
kind: doc
target_file: src/ide_app/dialogs/inherit_form.rs
line_hint: 1
author:
created:
title: "IdeApp::atender_inherit_form"
tags: []
signature: "pub (in super :: super) fn atender_inherit_form (& mut self , cx : & mut Cx , actions : & Actions)"
---

Turns the dialog's clicks and typing into changes of its model (#851).

Create (or Enter in the name) writes the form and opens it in the designer;
Cancel, the X and Esc close the dialog writing nothing.
