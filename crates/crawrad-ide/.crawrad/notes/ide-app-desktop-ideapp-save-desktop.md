---
id: ide-app-desktop-ideapp-save-desktop
kind: doc
target_file: src/ide_app/desktop.rs
line_hint: 30
author: 
created: 
title: "IdeApp::save_desktop"
tags: []
signature: "pub (super) fn save_desktop (& mut self , cx : & mut Cx)"
---

Saves the project desktop state when configured.

Writes the open project's desktop when Options keeps it. Silent, as
the dock's own file is: a failure loses the arrangement, not work.
