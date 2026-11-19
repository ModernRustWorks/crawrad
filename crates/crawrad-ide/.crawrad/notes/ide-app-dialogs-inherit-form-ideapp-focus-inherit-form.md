---
id: ide-app-dialogs-inherit-form-ideapp-focus-inherit-form
kind: doc
target_file: src/ide_app/dialogs/inherit_form.rs
line_hint: 76
author: 
created: 
title: "IdeApp::focus_inherit_form"
tags: []
signature: "pub (in super :: super) fn focus_inherit_form (& mut self , cx : & mut Cx)"
---

Hands the keyboard to the New Inherited Form name field (#851).

The field has no area until the dialog is drawn, so while it has none this
asks for the next frame and tries again — the same wait as
`IdeApp::focus_quick_open`. The suggested name comes selected, so typing a
name replaces it: naming the form is the first thing the dialog asks.
