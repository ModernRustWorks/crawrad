---
id: ide-app-dialogs-quick-open-ideapp-focus-quick-open
kind: doc
target_file: src/ide_app/dialogs/quick_open.rs
line_hint: 101
author: 
created: 
title: "IdeApp::focus_quick_open"
tags: []
signature: "pub (in super :: super) fn focus_quick_open (& mut self , cx : & mut Cx)"
---

Focuses the Quick Open input field once drawn.

The field takes the keys once it has been DRAWN: a dialog shown in
this pass has no area yet, and a focus set on it goes nowhere (the
`Enter` of #256 was lost so). Retried on the next events until then.
