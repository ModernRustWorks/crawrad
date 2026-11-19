---
id: ide-app-chrome-ideapp-with-menu-context
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 234
author: 
created: 
title: "IdeApp::with_menu_context"
tags: []
signature: "fn with_menu_context (& self , f : impl FnOnce (& crawrad_actions :: Context) -> Vec < menu :: Row > ,) -> Vec < menu :: Row >"
---

Runs a closure with the active menu actions context.

Runs `f` with the context a menu row is judged in (enabled, ticked);
no registry yet, no rows.
