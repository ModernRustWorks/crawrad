---
id: ide-app-dialogs-color-ideapp-handle-color
kind: doc
target_file: src/ide_app/dialogs/color.rs
line_hint: 119
author: 
created: 
title: "IdeApp::handle_color"
tags: []
signature: "pub (in super :: super) fn handle_color (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles gestures and input for the color dialog.

The dialog's gestures: the square, the strip, a patch, a field's Enter,
OK and Cancel (or Escape).

## Inside `handle_color`: `if moved || said.is_some() {`

The fields are written when the colour moved, never while one is
typed in: a redraw that wrote them would erase what is being typed.
