---
id: ide-app-dialogs-exit-ideapp-draw-quit
kind: doc
target_file: src/ide_app/dialogs/exit.rs
line_hint: 20
author: 
created: 
title: "IdeApp::draw_quit"
tags: []
signature: "pub (in super :: super) fn draw_quit (& mut self , cx : & mut Cx)"
---

Writes the exit dialog to the screen.

**It arises from the same question that opened it**, and not from a snapshot: if
anything saves while it is open, it disappears on its own — which is correct,
because the question ceased to exist.
