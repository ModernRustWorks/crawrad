---
id: ide-app-rename-ideapp-draw-rename
kind: doc
target_file: src/ide_app/rename.rs
line_hint: 237
author: 
created: 
title: "IdeApp::draw_rename"
tags: []
signature: "pub (super) fn draw_rename (& mut self , cx : & mut Cx)"
---

Renders or hides the rename symbol dialog and updates focus state.

The dialog on screen, or hidden when closed — and then the code
editor takes the keys back, as in a modern editor: Ctrl+Z right after Apply
undoes the rename.
