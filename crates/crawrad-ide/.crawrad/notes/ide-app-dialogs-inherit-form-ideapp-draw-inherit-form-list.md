---
id: ide-app-dialogs-inherit-form-ideapp-draw-inherit-form-list
kind: doc
target_file: src/ide_app/dialogs/inherit_form.rs
line_hint: 1
author:
created:
title: "IdeApp::draw_inherit_form_list"
tags: []
signature: "pub (in super :: super) fn draw_inherit_form_list (& self , cx : & mut Cx2d , lista : & mut PortalList)"
---

Draws the dialog's list of forms, each derived one indented under its base.

The picked row is highlighted with an orange edge; a derived row says what
it inherits, a top-level one says where it lives.
