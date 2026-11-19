---
id: ide-app-dialogs-exit-ideapp-close-current-project
kind: doc
target_file: src/ide_app/dialogs/exit.rs
line_hint: 0
author: 
created: 
title: "IdeApp::close_current_project"
tags: []
---

Clears what the previous project left on screen.

Its saved editor tabs and its form close; unsaved buffers stay, nothing typed is
dropped here. The desktop is saved first and its root forgotten, so the next save
cannot overwrite it with the new project's tabs.
