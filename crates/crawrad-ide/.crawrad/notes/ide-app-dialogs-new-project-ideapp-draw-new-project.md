---
id: ide-app-dialogs-new-project-ideapp-draw-new-project
kind: doc
target_file: src/ide_app/dialogs/new_project.rs
line_hint: 171
author: 
created: 
title: "IdeApp::draw_new_project"
tags: []
signature: "pub (in super :: super) fn draw_new_project (& mut self , cx : & mut Cx)"
---

Escreve o `New Project…` na tela com galeria, filtros e preview.

## Inside `draw_new_project`: `let with_fw = tpl.supports_framework;`

#491: the UI framework, only for a template with a window; the
description, the structure, the options and the button follow it
