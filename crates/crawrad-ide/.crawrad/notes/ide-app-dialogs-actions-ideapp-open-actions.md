---
id: ide-app-dialogs-actions-ideapp-open-actions
kind: doc
target_file: src/ide_app/dialogs/actions.rs
line_hint: 11
author: 
created: 
title: "IdeApp::open_actions"
tags: []
signature: "pub (in super :: super) fn open_actions (& mut self , cx : & mut Cx , lista : NodeId)"
---

Opens the action list editor.

The chosen category starts at `(All)` — showing everything is what a classic RAD IDE
does, and it is the only filter that does not hide what the user just created.
