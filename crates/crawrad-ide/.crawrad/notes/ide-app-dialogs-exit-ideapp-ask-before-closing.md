---
id: ide-app-dialogs-exit-ideapp-ask-before-closing
kind: doc
target_file: src/ide_app/dialogs/exit.rs
line_hint: 40
author: 
created: 
title: "IdeApp::ask_before_closing"
tags: []
signature: "pub (in super :: super) fn ask_before_closing (& mut self , cx : & mut Cx , event : & Event)"
---

Intercepts window close requests to prompt for unsaved changes.

**The system's close asks too** (Alt+F4, the taskbar), as the title
bar's X does: with work to save, the close is refused and the same
dialog opens. The IDE's own close (`CloseWindow`) destroys the window
without this request, so the dialog's Save and Discard still close.
