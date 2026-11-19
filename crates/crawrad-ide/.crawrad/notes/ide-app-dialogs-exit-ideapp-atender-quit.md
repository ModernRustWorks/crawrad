---
id: ide-app-dialogs-exit-ideapp-atender-quit
kind: doc
target_file: src/ide_app/dialogs/exit.rs
line_hint: 53
author: 
created: 
title: "IdeApp::atender_quit"
tags: []
signature: "pub (in super :: super) fn atender_quit (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles the three outcomes.

`Save` writes and closes, `Discard` closes, `Cancel` aborts — and aborting is
what happens by default, because the worst possible outcome here is losing
work without choosing to.

## Inside `atender_quit`: `let salvar = dlg.button(id!(quit_save)).clicked(actions) && self.sa...`

Save is Save All, and it closes only when everything was written:
a failed file stays dirty and the dialog stays, saying which.
