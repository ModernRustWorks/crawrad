---
id: ide-app-dialogs-options-ideapp-draw-options
kind: doc
target_file: src/ide_app/dialogs/options.rs
line_hint: 144
author: 
created: 
title: "IdeApp::draw_options"
tags: []
signature: "pub (in super :: super) fn draw_options (& mut self , cx : & mut Cx)"
---

Draws the Options dialog and active page.

Draws Options. The page is chosen by VISIBILITY, not by template: a
template switch would recreate the widgets and lose what is being typed.
