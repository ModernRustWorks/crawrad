---
id: ide-app-dialogs-exit-ideapp-after-saving
kind: doc
target_file: src/ide_app/dialogs/exit.rs
line_hint: 0
author: 
created: 
title: "IdeApp::after_saving"
tags: []
---

Asks about unsaved work before a project switch.

File › New Project and Open Project close the project on screen, so they use the
same Unsaved changes dialog closing the IDE uses; with nothing to save they go on.
