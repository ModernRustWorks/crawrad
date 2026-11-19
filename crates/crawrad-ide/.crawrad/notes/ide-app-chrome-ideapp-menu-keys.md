---
id: ide-app-chrome-ideapp-menu-keys
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 446
author: 
created: 
title: "IdeApp::menu_keys"
tags: []
signature: "pub (super) fn menu_keys (& mut self , cx : & mut Cx , event : & Event) -> bool"
---

Handles keyboard navigation when a menu is open.

With a menu open its keys are the menu's (#257): Up/Down move over the
rows that can run (rules and muted rows skipped), Enter runs the
highlighted one. Esc keeps closing it in `tecla`. With a submenu open
the keys are the submenu's, and Left or Esc closes only it; Right opens
the submenu of the highlighted row (#558).
