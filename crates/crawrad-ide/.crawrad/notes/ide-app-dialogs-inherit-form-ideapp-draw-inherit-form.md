---
id: ide-app-dialogs-inherit-form-ideapp-draw-inherit-form
kind: doc
target_file: src/ide_app/dialogs/inherit_form.rs
line_hint: 1
author:
created:
title: "IdeApp::draw_inherit_form"
tags: []
signature: "pub (in super :: super) fn draw_inherit_form (& mut self , cx : & mut Cx)"
---

Paints the New Inherited Form dialog from its model (#851).

The preview is a second design canvas showing the picked base, scaled to
fit; it is reopened only when the base changes, not on every frame.
