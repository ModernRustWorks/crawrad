---
id: ide-app-desktop-ideapp-restore-desktop
kind: doc
target_file: src/ide_app/desktop.rs
line_hint: 108
author: 
created: 
title: "IdeApp::restore_desktop"
tags: []
signature: "pub (super) fn restore_desktop (& mut self , cx : & mut Cx , root : & std :: path :: Path , startup : bool)"
---

Restores the project desktop docks, tabs and window placement.

Opens `root` as the project whose desktop is kept, giving back the
docks, the tabs (see [`Self::restore_desktop_tabs`]) and, at startup
only, the window.

## Inside `restore_desktop`: `if let Some(arranjo) = layout::decode(&d.dock).filter(|_| startup) {`

The dock and the window only at startup, before anything is drawn:
a dock reloaded later remakes the editor under its open tabs.
